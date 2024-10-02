FROM alpine:3.15

RUN apk --no-cache add postgresql12-client
RUN apk add --no-cache bash

COPY tls /var/lib/postgresql/certs
RUN chown 999 /var/lib/postgresql/certs
RUN chown 999 /var/lib/postgresql/certs -Rf
RUN chmod 600 /var/lib/postgresql/certs/*

COPY scripts /var/lib/postgresql/scripts
RUN chmod a+x /var/lib/postgresql/scripts/*.sh

WORKDIR /var/lib/postgresql/scripts/

CMD [ "sleep", "50000000000" ]