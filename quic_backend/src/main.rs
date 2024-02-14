use std::{
    fs::{self, File},
    io::{self, prelude::*},
    path::{self},
};
use serde_json::Value;

use anyhow::Context;
use log::{info, error};

use bytes::Bytes;
use clap::Parser;

// use rustls::pki_types::PrivateKeyDer;
use rustls::{Certificate, PrivateKey};
use webtransport_quinn::Session;

// https://stackoverflow.com/questions/73429672/how-do-i-import-a-file-from-a-folder-in-main
// use server_config ::*;
use routes::*;
use controller::*;
use dto::*;
use data::*;

// mod server_config;
mod routes;
mod controller;
mod dto;
mod data;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "[::]:4443")]
    addr: std::net::SocketAddr,

    /// Use the certificates at this path, encoded as PEM.
    #[arg(long, default_value = "../cert/localhost.crt")]
    pub tls_cert: path::PathBuf,

    /// Use the private key at this path, encoded as PEM.
    #[arg(long, default_value = "../cert/localhost.key")]
    pub tls_key: path::PathBuf,

    #[arg(long, default_value = "../data")]
    pub data_path: path::PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()>  {
    // Enable info logging.
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::init_from_env(env);

    let args = Args::parse();

    // Create data path
    {
        let root = args.data_path.clone();
        let root2 = root.as_path();
        // This is where they save their files. 
        let _ = fs::create_dir_all(root2.join("template"));
        let _ = fs::create_dir_all(root2.join("projects"));
        // We define some sample. 
        let _ = fs::create_dir_all(root2.join("sample_proj"));
        let _ = fs::create_dir_all(root2.join("sample_templ"));
    }

    // Think of how to re-read cert after renew without restarting server later. 
    // Read the PEM certificate chain
    let chain = fs::File::open(args.tls_cert.clone()).context("failed to open cert file")?;
    let mut chain = io::BufReader::new(chain);

    let chain: Vec<Certificate> = rustls_pemfile::certs(&mut chain)?
        .into_iter()
        .map(Certificate)
        .collect();
    // let chain = rustls_pemfile::certs(&mut chain)
    //     .collect::<Result<Vec<_>, _>>()
    //     .unwrap();

    anyhow::ensure!(!chain.is_empty(), "could not find certificate");

    // Read the PEM private key
    let keys = fs::File::open(args.tls_key).context("failed to open key file")?;
    let mut keychain = io::BufReader::new(keys);

    // Read the keys into a Vec so we can parse it twice.
    // let mut buf = Vec::new();
    // keys.read_to_end(&mut buf)?;

    // Try to parse a PKCS#8 key
    // -----BEGIN PRIVATE KEY-----
    // let mut keys = rustls_pemfile::pkcs8_private_keys(&mut keychain)
    //     .collect::<Result<Vec<_>, _>>()
    //     .unwrap();
    let mut keys: Vec<PrivateKey> = rustls_pemfile::pkcs8_private_keys(&mut keychain)?
        .into_iter()
        .map(PrivateKey)
        .collect();

    // // We can't assign it back to `keys` because it's of a different type since v0.22. 
    // let mut keys = rustls_pemfile::ec_private_keys(&mut keychain)
    //     .collect::<Result<Vec<_>, _>>()
    //     .unwrap();
    
    anyhow::ensure!(!keys.is_empty(), "could not find private key");
    anyhow::ensure!(keys.len() < 2, "expected a single key");

    let key = keys.remove(0);
    // let key: PrivateKeyDer<'_> = keys.remove(0).try_into().unwrap();

    // Standard Quinn setup (updated, test if it works)
    let mut tls_config = rustls::ServerConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_protocol_versions(&[&rustls::version::TLS13])
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(chain, key)?;
    // let mut tls_config = rustls::ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
    //     .with_no_client_auth()
    //     .with_single_cert(chain, key)?;

    tls_config.max_early_data_size = u32::MAX;
    tls_config.alpn_protocols = vec![webtransport_quinn::ALPN.to_vec()]; // this one is important
    
    let config = quinn::ServerConfig::with_crypto(std::sync::Arc::new(tls_config));

    info!("listening on {}", args.addr);

    let server = quinn::Endpoint::server(config, args.addr)?;

    // Accept new connections.
    while let Some(conn) = server.accept().await {
        tokio::spawn(async move {
            let err = run_conn(conn).await;
            if let Err(err) = err {
                error!("connection failed: {}", err)
            }
        });
    }

    Ok(())
}

async fn run_conn(conn: quinn::Connecting) ->  anyhow::Result<()> {
    // Waiting for QUIC handshake.
    let conn = conn.await.context("Failed to accept QUIC connection.")?;
    // Perform WebTransport Handshake.
    let request = webtransport_quinn::accept(conn.clone()).await?;
    info!("Request: {}", request.url());

    let path = request.url().clone().path().to_owned();

    // Accept session. 
    let session = request.ok().await.context("Failed to accept session")?;

    if let Err(err) = run_session(session, path).await {
        info!("closing session: {}", err);
    }

    Ok(())
}

async fn run_session(session: Session, path: String) -> anyhow::Result<()> {
    loop {
        tokio::select! {
            res = session.accept_bi() => {
                let (mut send, mut recv) = res?;

                let msg = recv.read_to_end(8192).await?;
                // handle_bi(send, recv, path.clone()).await;
                match routes_handler(msg.try_into().unwrap(), path.clone()) {
                    Ok(Some(value)) => send.write_all(value.as_bytes()).await?,
                    Err(err) => send.write_all(err.as_bytes()).await?,
                    _ => {}  // all none no need send anything back. 
                }
            },
            res = session.read_datagram() => {
                let msg = res?;

                // handle_datagram(session.clone(), msg, path.clone()).await;
                match routes_handler(msg, path.clone()) {
                    Ok(Some(value)) => session.send_datagram(value.try_into().unwrap()).await?,
                    Err(err) => session.send_datagram(err.try_into().unwrap()).await?,
                    _ => {}
                }
            }
        }
    }
}


