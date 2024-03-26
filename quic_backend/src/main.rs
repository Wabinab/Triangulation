use std::{
    collections::HashMap, fs::{self, File}, io::{self, prelude::*}, 
    path::{self, Path, PathBuf}, time::Duration
};
use quinn::ServerConfig;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use anyhow::Context;
use log::{info, error};

use bytes::Bytes;
use clap::Parser;
use time::OffsetDateTime;
use ring::digest::{digest, SHA256};

// use rustls::pki_types::PrivateKeyDer;
use rustls::{Certificate, PrivateKey};
use webtransport_quinn::Session;

// mod models;
// mod schema;

// https://stackoverflow.com/questions/73429672/how-do-i-import-a-file-from-a-folder-in-main
// use server_config ::*;
use routes::*;
use controller::*;
use dto::*;
use data::*;
use types::*;

// mod server_config;
mod routes;
mod controller;
mod dto;
mod data;
mod types;
mod messages;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "[::]:4443")]
    addr: std::net::SocketAddr,

    #[arg(long, default_value = "../cert")]
    pub cert_path: PathBuf,

    /// Use the certificates at this path, encoded as PEM.
    #[arg(long, default_value = "../cert/localhost.crt")]
    pub tls_cert: path::PathBuf,

    /// Use the private key at this path, encoded as PEM.
    #[arg(long, default_value = "../cert/localhost.key")]
    pub tls_key: path::PathBuf,

    #[arg(long, default_value = "../data")]
    pub data_path: path::PathBuf,

    /// Move localhost.hex to angular path
    #[arg(long, default_value = "../client-app/src/assets")]
    pub ng_asset_path: path::PathBuf,
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
        let _ = fs::create_dir_all(root2.join("project"));
        // We define some sample. 
        let _ = fs::create_dir_all(root2.join("sample_proj"));
        let _ = fs::create_dir_all(root2.join("sample_templ"));
    }

    // let mut g = true;
    // while g {
    start_server(Args::parse()).await?;
    //   info!("is_renew = {}", g);
    //   thread::sleep(std::time::Duration::from_secs(3));
    //   info!("Finish sleeping.");
    // }

    Ok(())
}

async fn start_server(args: Args) -> anyhow::Result<()> {
    
    let init_config = gen_config(Args::parse())?;
    info!("listening on {}", args.addr);

    let server = quinn::Endpoint::server(init_config, args.addr)?;

    // Accept new connections.
    while let Some(conn) = server.accept().await {
        if check_and_renew(args.cert_path.as_path(), args.ng_asset_path.as_path()) { 
          let config = gen_config(Args::parse())?;
          server.set_server_config(Some(config));
          info!("Set new server config");
          // break; 
        }
        tokio::spawn(async move {    
            let err = run_conn(conn).await;
            if let Err(err) = err {
                error!("connection failed: {}", err)
            }
        });
    }

    // server.close(VarInt::from_u32(900), "Renewing Server Certificate".to_string().as_bytes());
    Ok(())
}

fn gen_config(args: Args) -> anyhow::Result<ServerConfig> {
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
    return Ok(config);
}


async fn run_conn(conn: quinn::Connecting) ->  anyhow::Result<()> {
    // Waiting for QUIC handshake.
    // info!("{:#?}", conn.conn);

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
                let args = Args::parse();

                let msg = recv.read_to_end(8192).await?;
                match routes_handler(msg.try_into().unwrap(), path.clone(), args.data_path) {
                    Ok(Some(value)) => send.write_all(value.as_bytes()).await?,
                    Err(err) => send.write_all(jsonify_err(err).as_bytes()).await?,
                    _ => {}  // all none no need send anything back. 
                }
            },
            res = session.read_datagram() => {
                let msg = res?;
                let args = Args::parse();

                // handle_datagram(session.clone(), msg, path.clone()).await;
                match routes_handler(msg, path.clone(), args.data_path) {
                    Ok(Some(value)) => session.send_datagram(value.try_into().unwrap()).await?,
                    Err(err) => session.send_datagram(jsonify_err(err).try_into().unwrap()).await?,
                    _ => {}
                }
            }
        }
    }
}


// ==================================================
// All unwrap() are left as it is; because if it fails, we'll need to debug, so no need
// alternative error message. 

/// Check for certificate expiry. If expired, call `renew_cert`. 
/// If can't find cert_param.json, also call `renew_cert.`
/// return "true" if renewed.
fn check_and_renew(root: &Path, ng_path: &Path) -> bool {
  let path = root.join("cert_param.json");
  let read_data_opt = fs::read(path);
  if read_data_opt.is_err() {
      return renew_cert(root, ng_path);
  }
  let read_data = read_data_opt.unwrap();
  let data = String::from_utf8_lossy(&read_data).into_owned();
  let serded = serde_json::from_str::<HashMap<String, i64>>(&data).unwrap();

  let difference = OffsetDateTime::now_utc().unix_timestamp()
    .checked_sub(serded.get("not_after").unwrap().to_owned()).unwrap();
  if difference >= -1 * 60 * 60 * 24 { return renew_cert(root, ng_path); }  // one day before expiry. 
  // return false;
  return false;  // for debug purposes. 
}

/// renew certificate.
/// Always return true at end. 
fn renew_cert(root: &Path, ng_path: &Path) -> bool {
  info!("Certificate expired. Renewing!");
  let mut cert_params =
      rcgen::CertificateParams::new(vec!["localhost".to_string(), "127.0.0.1".to_string()]);

  let now = OffsetDateTime::now_utc();
  cert_params.not_before = now;
  cert_params.not_after = now + Duration::from_secs(60 * 60 * 24 * 14); // 10 days

  cert_params.alg = &rcgen::PKCS_ECDSA_P256_SHA256;

  // Save cert params to file
  let mut file = File::create(root.join("cert_param.json")).unwrap();
  let data = HashMap::from([
    ("not_before", cert_params.not_before.unix_timestamp()),
    ("not_after", cert_params.not_after.unix_timestamp())
  ]);
  let json = serde_json::to_string(&data).unwrap();
  file.write_all(json.as_bytes()).unwrap();

  let cert = rcgen::Certificate::from_params(cert_params).unwrap();

  let cert_pem = cert.serialize_pem().unwrap();
  println!("{}", cert_pem);
  let mut file = File::create(root.join("localhost.crt")).unwrap();
  file.write_all(cert_pem.as_bytes()).unwrap();

  let priv_pem = cert.serialize_private_key_pem();
  println!("{}", priv_pem);
  let mut file = File::create(root.join("localhost.key")).unwrap();
  file.write_all(priv_pem.as_bytes()).unwrap();

  let mut cert_reader = std::io::BufReader::new(cert_pem.as_bytes());
  let certs = rustls_pemfile::certs(&mut cert_reader).unwrap();
  let cert_der = certs.first().expect("No certificate found");
  let fingerprint = digest(&SHA256, cert_der.as_ref());
  let fingerprint_hex = hex::encode(fingerprint.as_ref());

  println!("Fingerprint {}", fingerprint_hex);
  let mut file = File::create(root.join("localhost.hex")).unwrap();
  file.write_all(fingerprint_hex.as_bytes()).unwrap();
  // Make a copy at angular assets folder. 
  let mut file = File::create(ng_path.join("localhost.hex")).unwrap();
  file.write_all(fingerprint_hex.as_bytes()).unwrap();

  // Call function to restart
  return true;
}

// =========================================================
// Deal with error
fn jsonify_err(err: String) -> String {
  json!({
    "err": err
  }).to_string()
}


#[cfg(test)]
mod test;