mod command_arguments;
mod tls_client_config;
mod tls_server_config;
mod tracing_setup;

use crate::command_arguments::CommandArguments;
use anyhow::{anyhow, bail};
use clap::Parser;
use rustls::pki_types::ServerName;
use rustls::ClientConfig;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use tokio::io::{self, split, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::rustls::{self, ServerConfig};
use tokio_rustls::{TlsAcceptor, TlsConnector, TlsStream};
use tracing::Instrument;
use uuid::Uuid;

// References:
// https://postgresconf.org/system/events/document/000/000/183/pgconf_us_v4.pdf
// https://www.tzeejay.com/blog/2022/06/golang-postgresql-check-certificates
// https://www.postgresql.org/docs/current/ssl-tcp.html
// https://www.postgresql.org/docs/current/libpq-ssl.html
// https://xnuter.medium.com/writing-a-modern-http-s-tunnel-in-rust-56e70d898700
// https://ocw.mit.edu/courses/6-875-cryptography-and-cryptanalysis-spring-2005/
// https://tailscale.com/blog/introducing-pgproxy
// AWS - Aurora / RDS: https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/UsingWithRDS.SSL.html
// Google - Cloud SQL: https://github.com/brianc/node-postgres-docs/issues/79#issuecomment-1553759056

#[tokio::main]
#[tracing::instrument(name = "pgt_proxy")]
async fn main() -> anyhow::Result<()> {
    let args: CommandArguments = CommandArguments::parse();
    // We observed that the program would output nothing (stdout/stderr) upon tracing init failure,
    // when using stderr as the writer.
    // Let's panic when we fail to initialize tracing, which will surely print to stderr.
    tracing_setup::init(args.log_level).expect("Failed to initialize tracing.");
    tracing::info!("Hello from PGT Proxy. Starting up!");

    tracing::info!("Fetching Server Config.");
    let tls_server_config = tls_server_config::server_config(
        &args.server_certificate_path,
        &args.server_private_key_path,
    )?;
    tracing::info!("Fetched Server Config.");
    tracing::info!("Fetching Client Config.");
    let tls_client_config = tls_client_config::client_config(&args.client_ca_roots_path)?;
    tracing::info!("Fetched Client Config.");

    let listener = TcpListener::bind(format!("0.0.0.0:{}", &args.server_port)).await?;
    tracing::info!(port = ?args.server_port, "Listening");
    while let Ok((inbound_tcp_stream, _)) = listener.accept().await {
        let request_id = Uuid::new_v4().to_string();

        let request_id_for_task = request_id.clone();
        let task = tokio::spawn(
            handle_inbound_request(
                inbound_tcp_stream,
                tls_server_config.clone(),
                tls_client_config.clone(),
                args.client_connection_host_or_ip.to_owned(),
                args.client_connection_port.to_owned(),
                args.client_tls_validation_host.to_owned(),
                request_id_for_task,
            )
            .in_current_span(),
        );

        let request_id_for_join = request_id.clone();
        tokio::spawn(async move {
            match task.await {
                Ok(Ok(())) => {
                    tracing::info!(request_id = %request_id_for_join, "Task completed successfully.");
                }
                Ok(Err(e)) => {
                    tracing::error!(
                        ?e,
                        request_id = %request_id_for_join,
                        "Task failed with an error."
                    );
                }
                Err(e) => {
                    tracing::error!(
                        ?e,
                        request_id = %request_id_for_join,
                        "Task panicked or was cancelled."
                    );
                }
            }
        });
    }

    bail!("Something went wrong with the listener! Exiting program.")
}

#[tracing::instrument(
    skip_all,
    fields(
        addr = connection_host_or_ip,
        port = connection_port,
        request_id = request_id,
    )
)]
async fn handle_inbound_request(
    inbound_stream: TcpStream,
    server_config: ServerConfig,
    client_config: ClientConfig,
    connection_host_or_ip: String,
    connection_port: String,
    tls_validation_host: String,
    request_id: String,
) -> anyhow::Result<()> {
    tracing::info!(
        ?request_id,
        "Accepting inbound connection from PG client. Proceeding to handshake.",
    );
    let inbound_tls_stream = inbound_handshake(inbound_stream, server_config, &request_id).await?;

    tracing::info!(
        ?request_id,
        "Inbound TLS OK. Proceeding to outbound connection to PG server.",
    );
    let outbound_tls_stream = outbound_handshake(
        &connection_host_or_ip,
        &connection_port,
        client_config,
        &tls_validation_host,
        &request_id,
    )
    .await?;

    tracing::info!(
        ?request_id,
        "Outbound TLS OK. Proceeding to join inbound and outbound connection.",
    );
    join(inbound_tls_stream, outbound_tls_stream, &request_id).await?;

    Ok(())
}

#[tracing::instrument(skip_all)]
async fn inbound_handshake(
    mut inbound_stream: TcpStream,
    server_config: ServerConfig,
    request_id: &str,
) -> anyhow::Result<TlsStream<TcpStream>> {
    let mut buffer = [0u8; 8];
    inbound_stream.read_exact(&mut buffer).await?;
    if !buffer.starts_with(&[0, 0, 0, 8, 4, 210, 22, 47]) {
        // tell pgClient we do not support plaintext connections
        inbound_stream.write_all(b"N").await?;
        let err_msg = "TLS not supported by PG client on inbound connection";
        tracing::error!("{err_msg}");
        bail!("{err_msg}. RequestId: {request_id}");
    }
    // tell pgClient we're proceeding with TLS
    inbound_stream.write_all(b"S").await?;

    let stream = TlsAcceptor::from(Arc::new(server_config))
        .accept(inbound_stream)
        .await?
        .into();

    Ok(stream)
}

#[tracing::instrument(skip_all)]
async fn outbound_handshake(
    connection_host_or_ip: &str,
    connection_port: &str,
    client_config: ClientConfig,
    tls_validation_host: &str,
    request_id: &str,
) -> anyhow::Result<TlsStream<TcpStream>> {
    let connect_to = format!("{}:{}", connection_host_or_ip, connection_port);
    let connect_to = connect_to
        .to_socket_addrs()?
        .next()
        .ok_or(anyhow!("Invalid address: {connect_to:?}"))?;
    let mut outbound_stream = TcpStream::connect(connect_to).await?;
    // tell pgServer we only support TLS connections
    outbound_stream
        .write_all(&[0, 0, 0, 8, 4, 210, 22, 47])
        .await?;
    let mut buffer = [0u8; 1];
    outbound_stream.read_exact(&mut buffer).await?;
    if !buffer.starts_with(b"S") {
        bail!(
            "TLS not supported by PG server on outbound connection. RequestId: {}",
            request_id
        );
    }

    let stream = TlsConnector::from(Arc::new(client_config))
        .connect(
            ServerName::DnsName(tls_validation_host.to_owned().try_into()?),
            outbound_stream,
        ) // tls verification for pgServer happens here
        .await?
        .into();

    Ok(stream)
}

#[tracing::instrument(skip_all)]
async fn join(
    inbound: TlsStream<TcpStream>,
    outbound: TlsStream<TcpStream>,
    request_id: &str,
) -> anyhow::Result<()> {
    let (mut ir, mut iw) = split(inbound);
    let (mut or, mut ow) = split(outbound);

    let result = tokio::try_join!(io::copy(&mut ir, &mut ow), io::copy(&mut or, &mut iw));

    match result {
        Ok(_) => {
            tracing::info!(?request_id, "Connection closed gracefully.");
            Ok(())
        }
        Err(e) => {
            tracing::error!(?e, ?request_id, "Connection aborted due to an error.");
            Err(e.into())
        }
    }
}
