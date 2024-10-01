use anyhow::{anyhow, bail};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use std::fs::File;
use std::io::BufReader;

pub(crate) fn server_config(
    cert_path: &str,
    private_key_path: &str,
) -> anyhow::Result<ServerConfig> {
    let config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            server_config_load_certificates(cert_path)?,
            server_config_load_private_key(private_key_path)?,
        )?;

    println!("Loaded TLS files for inbound connections.");

    Ok(config)
}

fn server_config_load_certificates(path: &str) -> anyhow::Result<Vec<CertificateDer<'static>>> {
    let mut certs = Vec::new();
    let mut buf = BufReader::new(File::open(path)?);
    for cert_result in rustls_pemfile::certs(&mut buf) {
        let cert = cert_result?;
        certs.push(cert);
    }
    if certs.is_empty() {
        bail!("No certificates found in cert file");
    }
    Ok(certs)
}

fn server_config_load_private_key(path: &str) -> anyhow::Result<PrivateKeyDer<'static>> {
    let mut buf = BufReader::new(File::open(path)?);
    let priv_key_opt = rustls_pemfile::private_key(&mut buf)?;
    let priv_key = priv_key_opt.ok_or(anyhow!("Private key not found in path {path:?}"))?;
    Ok(priv_key)
}
