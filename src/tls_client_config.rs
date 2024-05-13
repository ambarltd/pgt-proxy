use rustls::{ClientConfig, RootCertStore};
use std::fs;
use std::io::{Cursor, Read};

pub(crate) fn client_config(ca_roots_directory: &str) -> ClientConfig {
    let mut root_cert_store = RootCertStore::empty();
    let read_dir = fs::read_dir(ca_roots_directory).unwrap();

    for r in read_dir {
        let dir_entry = r.unwrap();
        let path = dir_entry.path();

        if path.is_file() && path.extension().map_or(false, |e| e == "pem") {
            let mut file = fs::File::open(&path).unwrap();
            let mut pem_data = Vec::new();
            file.read_to_end(&mut pem_data).unwrap();

            let mut reader = Cursor::new(pem_data);
            for cert in rustls_pemfile::certs(&mut reader) {
                root_cert_store.add(cert.unwrap()).unwrap();
            }
        }
    }

    let config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();

    println!("Loaded TLS files for outbound connections.");

    config
}
