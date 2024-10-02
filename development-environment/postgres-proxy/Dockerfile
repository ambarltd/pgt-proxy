FROM rust:1.81.0-bookworm

# Certs for the server
RUN mkdir -p /var/lib/postgresql/certs
COPY tls /var/lib/postgresql/certs

RUN chown 999 /var/lib/postgresql/certs
RUN chown 999 /var/lib/postgresql/certs -Rf
RUN chmod 600 /var/lib/postgresql/certs/client/*
RUN chmod 600 /var/lib/postgresql/certs/server/*

# Allows us to rerun tests, without redownloading crates.
RUN mkdir /code/src -p
WORKDIR /code
COPY code_copy/Cargo.toml /code/Cargo.toml
COPY code_copy/Cargo.lock /code/Cargo.lock
RUN echo "fn main() {}" > /code/src/main.rs
RUN cargo build --release

COPY code_copy/src/* /code/src/
RUN sleep 2 # ensures we detect changes to main.rs so that we recompile
RUN touch /code/src/main.rs
RUN cargo build --release

CMD ["/code/target/release/pgt_proxy", "--server-private-key-path", "/var/lib/postgresql/certs/server/my-public-domain.example.key", \
    "--server-certificate-path", "/var/lib/postgresql/certs/server/my-public-domain.example.crt.pem", \
    "--server-port", "5432", \
    "--client-connection-host-or-ip", "172.35.0.102", \
    "--client-connection-port", "5432", \
    "--client-tls-validation-host", "my-private-domain.example", \
    "--client-ca-roots-path", "/var/lib/postgresql/certs/client" \
]


