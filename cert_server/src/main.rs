use std::{
    collections::HashMap,
    convert::Infallible,
    fs::{self, File}, 
    io::Write,
    net::SocketAddr,
    path::{self, Path, PathBuf},
    time::Duration
};
use serde_json;
use ring::digest::{digest, SHA256};
use time::OffsetDateTime;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use clap::Parser;
use log::{info, error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "[::]:4443")]
    addr: std::net::SocketAddr,

    #[arg(long, default_value = "../cert")]
    pub cert_path: PathBuf,
}

async fn main_resp(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let args = Args::parse();
    let root = args.cert_path.as_path().clone(); 

    check_and_renew(root);

    let path = root.join("localhost.hex");
    let data = fs::read(&path).unwrap_or("Failed read content".to_owned().into_bytes());
    Ok(Response::new(Full::new(data.try_into().unwrap())))
}

/// Check for certificate expiry. If expired, call `renew_cert`. 
/// If can't find cert_param.json, also call `renew_cert.`
fn check_and_renew(root: &Path) {
    let path = root.join("cert_param.json");
    let read_data_opt = fs::read(path);
    if read_data_opt.is_err() {
        renew_cert(root);
        return;
    }
    let read_data = read_data_opt.unwrap();
    let data = String::from_utf8_lossy(&read_data).into_owned();
    let serded = serde_json::from_str::<HashMap<String, i64>>(&data).unwrap();
  
    let difference = OffsetDateTime::now_utc().unix_timestamp()
      .checked_sub(serded.get("not_after").unwrap().to_owned()).unwrap();
    if difference >= -1 * 60 * 60 * 24 {  renew_cert(root); }  // one day before expiry. 
}

/// renew certificate.
fn renew_cert(root: &Path) {
    info!("Certificate expiring. Renewing!");
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env = env_logger::Env::default().default_filter_or("info");
    env_logger::init_from_env(env);

    let args = Args::parse();
    let root = args.cert_path.clone();
    let _ = fs::create_dir_all(root);

    let addr = args.addr.clone();

    let listener = TcpListener::bind(addr).await?;
    info!("listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `main_resp` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(main_resp))
                .await
            {
                log::info!("Error serving connection: {:?}", err);
            }
        });
    }
}
