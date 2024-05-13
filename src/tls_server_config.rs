use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use std::fs::File;
use std::io::BufReader;

pub(crate) fn server_config(cert_path: &str, private_key_path: &str) -> ServerConfig {
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            server_config_load_certificates(cert_path),
            server_config_load_private_key(private_key_path),
        )
        .unwrap();

    println!("Loaded TLS files for inbound connections.");

    config
}

fn server_config_load_certificates(path: &str) -> Vec<CertificateDer<'static>> {
    let certs: Vec<CertificateDer> =
        rustls_pemfile::certs(&mut BufReader::new(File::open(path).unwrap()))
            .map(|c| c.unwrap())
            .collect();

    if certs.is_empty() {
        panic!("No certificates found in cert file");
    }

    certs
}

fn server_config_load_private_key(path: &str) -> PrivateKeyDer<'static> {
    rustls_pemfile::private_key(&mut BufReader::new(File::open(path.to_owned()).unwrap()))
        .unwrap()
        .unwrap()
}
