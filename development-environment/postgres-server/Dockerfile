FROM postgres:16.4

RUN mkdir -p /var/lib/postgresql/certs
COPY tls /var/lib/postgresql/certs

RUN chown 999 /var/lib/postgresql/certs
RUN chown 999 /var/lib/postgresql/certs -Rf
RUN chmod 600 /var/lib/postgresql/certs/*

CMD ["postgres", "-c", "wal_level=logical", \
     "-c", "ssl=on", \
     "-c", "ssl_cert_file=/var/lib/postgresql/certs/my-private-domain.example.crt.pem", \
     "-c", "ssl_key_file=/var/lib/postgresql/certs/my-private-domain.example.key"]