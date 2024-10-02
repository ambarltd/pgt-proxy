#!/bin/sh
set -e

PGPASSWORD=my_password psql "host=my-private-domain.example hostaddr=172.35.0.102 dbname=my_database user=my_username sslmode=verify-full sslrootcert=/var/lib/postgresql/certs/direct_connection/my-private-ca.example.crt.pem " --file 1_test_setup.sql
PGPASSWORD=my_password psql "host=my-public-domain.example hostaddr=172.35.0.103 dbname=my_database user=my_username sslmode=verify-full sslrootcert=/var/lib/postgresql/certs/proxy_connection/my-public-ca.example.crt.pem channel_binding=disable" --file 2_test_read.sql
