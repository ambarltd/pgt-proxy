use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(about = "Create a proxy server, which will use a client to connect to a postgres db.")]
pub(crate) struct CommandArguments {
    #[arg(
        long,
        help = "A path to a file containing the private key the server will use."
    )]
    pub(crate) server_private_key_path: String,
    #[arg(
        long,
        help = "A path to a file containing the certificate the server will use."
    )]
    pub(crate) server_certificate_path: String,
    #[arg(
        long,
        help = "A path to a file containing the port the server will bind to locally."
    )]
    pub(crate) server_port: String,
    #[arg(
        long,
        help = "A host or ip that the client will connect to, belonging to the postgres db."
    )]
    pub(crate) client_connection_host_or_ip: String,
    #[arg(
        long,
        help = "A port that the client will connect to, belonging to the postgres db."
    )]
    pub(crate) client_connection_port: String,
    #[arg(
        long,
        help = "The expected hostname in the TLS certificate belonging to the postgres db."
    )]
    pub(crate) client_tls_validation_host: String,
    #[arg(
        long,
        help = "A directory path that contains all the root certificate authorities the client should trust."
    )]
    pub(crate) client_ca_roots_path: String,
}
