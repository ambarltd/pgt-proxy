# TLS NOTES

Simply run the script `./test.sh`, to test the entire project.

### Notes about the generation of TLS files

```bash
mkdir -p all_tls_files
cd all_tls_files
rm * -Rf

# Generate public ca / domain TLS files
openssl genrsa -out my-public-ca.example.key 4096
openssl req -x509 -new -nodes -key my-public-ca.example.key -sha256 -days 3650 -out my-public-ca.example.crt \
-subj "/C=US/ST=State/L=City/O=My God CA/OU=Root CA/CN=my-public-ca.example" \
-addext "basicConstraints=critical,CA:true,pathlen:0" \
-addext "keyUsage=critical,keyCertSign,cRLSign,digitalSignature" \
-addext "subjectKeyIdentifier=hash" \
-addext "authorityKeyIdentifier=keyid:always,issuer"
openssl genrsa -out my-public-domain.example.key 2048
openssl req -new -key my-public-domain.example.key -out my-public-domain.example.csr \
-subj "/C=US/ST=State/L=City/O=My Domain/OU=Domain Cert/CN=my-public-domain.example"
openssl x509 -req -in my-public-domain.example.csr -CA my-public-ca.example.crt -CAkey my-public-ca.example.key -CAcreateserial \
-out my-public-domain.example.crt -days 365 -sha256 \
-extfile <(printf "basicConstraints=CA:FALSE\nkeyUsage=digitalSignature,keyEncipherment\nextendedKeyUsage=serverAuth,clientAuth\nsubjectAltName=DNS:my-public-domain.example")

# Generate private ca / domain TLS files
openssl genrsa -out my-private-ca.example.key 4096
openssl req -x509 -new -nodes -key my-private-ca.example.key -sha256 -days 3650 -out my-private-ca.example.crt \
-subj "/C=US/ST=State/L=City/O=My God CA/OU=Root CA/CN=my-private-ca.example" \
-addext "basicConstraints=critical,CA:true,pathlen:0" \
-addext "keyUsage=critical,keyCertSign,cRLSign,digitalSignature" \
-addext "subjectKeyIdentifier=hash" \
-addext "authorityKeyIdentifier=keyid:always,issuer"
openssl genrsa -out my-private-domain.example.key 2048
openssl req -new -key my-private-domain.example.key -out my-private-domain.example.csr \
-subj "/C=US/ST=State/L=City/O=My Domain/OU=Domain Cert/CN=my-private-domain.example"
openssl x509 -req -in my-private-domain.example.csr -CA my-private-ca.example.crt -CAkey my-private-ca.example.key -CAcreateserial \
-out my-private-domain.example.crt -days 365 -sha256 \
-extfile <(printf "basicConstraints=CA:FALSE\nkeyUsage=digitalSignature,keyEncipherment\nextendedKeyUsage=serverAuth,clientAuth\nsubjectAltName=DNS:my-private-domain.example")


cp my-private-ca.example.crt ../postgres-client/tls/direct_connection/my-private-ca.example.crt.pem
cp my-public-ca.example.crt ../postgres-client/tls/proxy_connection/my-public-ca.example.crt.pem
cp my-private-ca.example.crt ../postgres-proxy/tls/client/my-private-ca.example.crt.pem
cp my-public-domain.example.crt ../postgres-proxy/tls/server/my-public-domain.example.crt.pem
cp my-public-domain.example.key ../postgres-proxy/tls/server/my-public-domain.example.key
cp my-private-domain.example.crt ../postgres-server/tls/my-private-domain.example.crt.pem
cp my-private-domain.example.key ../postgres-server/tls/my-private-domain.example.key

```