#!/bin/sh
set -e

printf "\n"
echo "============================================"
echo "Setup starting"
echo "============================================"
echo "Requires sudo to delete any existing volumes"

docker compose down

mkdir postgres-proxy/code_copy/src -p
cp ../src/* postgres-proxy/code_copy/src/ -Rf
cp ../Cargo.toml postgres-proxy/code_copy/Cargo.toml
cp ../Cargo.lock postgres-proxy/code_copy/Cargo.lock

docker compose up -d --build --force-recreate

echo "Waiting for the containers to start. Sleeping for 15 more seconds."
sleep 5;
echo "Waiting for the containers to start. Sleeping for 10 more seconds."
sleep 5;
echo "Waiting for the containers to start. Sleeping for 5 more seconds."
sleep 5;

echo "============================================"
echo "Setup finished"
echo "============================================"

printf "\n\n\n\n\n"
echo "============================================"
echo "Executing end to end tests. Expecting to find records:"
echo "============================================"
docker exec -it pgt-proxy-postgres-client bash ./end_to_end_test.sh | grep "john.doe@example.com"
echo "============================================"
echo "Executed end to end tests successfully. Connected successfully to the database through the proxy, and read a record."
echo "============================================"

printf "\n\n\n\n\n"
echo "============================================"
echo "Checking the logs of the proxy container"
echo "============================================"
docker logs pgt-proxy-postgres-proxy 2>&1 | grep "Accepting inbound connection from PG client"
docker logs pgt-proxy-postgres-proxy 2>&1 | grep "Inbound TLS OK. Proceeding to outbound connection"
docker logs pgt-proxy-postgres-proxy 2>&1 | grep "Outbound TLS OK. Proceeding to join inbound and outbound"
docker logs pgt-proxy-postgres-proxy 2>&1 | grep "Connection closed gracefully"
docker logs pgt-proxy-postgres-proxy 2>&1 | grep "Task completed successfully"
echo "============================================"
echo "Checked the logs of the proxy container successfully"
echo "============================================"

printf "\n\n\n\n\n"
echo "All tests passed. Congratulations. Bye!"