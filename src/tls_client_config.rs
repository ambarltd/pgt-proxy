use rustls::{ClientConfig, RootCertStore};
use std::fs;
use std::io::{Cursor, Read};
use anyhow::bail;

pub(crate) fn client_config(ca_roots_directory: &str) -> anyhow::Result<ClientConfig> {
    let mut root_cert_store = RootCertStore::empty();
    let read_dir = fs::read_dir(ca_roots_directory)?;

    let mut root_ca_count = 0;
    for r in read_dir {
        let dir_entry = r?;
        let path = dir_entry.path();

        if path.is_file() && path.extension().map_or(false, |e| e == "pem") {
            let mut file = fs::File::open(&path)?;
            let mut pem_data = Vec::new();
            file.read_to_end(&mut pem_data)?;

            let mut reader = Cursor::new(pem_data);
            for cert_result in rustls_pemfile::certs(&mut reader) {
                root_ca_count += 1;
                let cert = cert_result?;
                root_cert_store.add(cert)?;
            }
        }
    }

    if root_ca_count == 0 {
        bail!("No root certificates found in directory: {ca_roots_directory:?}");
    }

    let config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    tracing::info!(
        ?ca_roots_directory,
        "Loaded TLS files for outbound connections."
    );

    Ok(config)
}
