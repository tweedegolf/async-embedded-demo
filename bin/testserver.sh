#!/usr/bin/env bash
set -eo pipefail

LOCAL_PORT=8088
PORT_FILE=target/PUBLIC_PORT
IP_ADDR_FILE=target/PUBLIC_IP_ADDR

mkdir -p target

echo "Setting up TCP tunnel with ngrok"
ngrok tcp $LOCAL_PORT 2>&1 > /dev/null &

URL=$(curl -s http://localhost:4040/api/tunnels | jq -r '.tunnels[0].public_url')
PARTS=($(basename $URL | tr ":" " "))
DOMAIN=${PARTS[0]}
PUBLIC_PORT=${PARTS[1]}
PUBLIC_IP_ADDR=$(dig +short $DOMAIN @resolver1.opendns.com)

echo -n $PUBLIC_PORT > $PORT_FILE
echo -n $PUBLIC_IP_ADDR > $IP_ADDR_FILE

echo "Found ngrok tunnel: $PUBLIC_IP_ADDR:$PUBLIC_PORT ($DOMAIN)"

while [ true ]
do
    echo "Listening on local port $LOCAL_PORT..."
    echo "===== START TCP INPUT ====="
    nc -l 0.0.0.0 $LOCAL_PORT
    echo "===== CONNECTION CLOSED ====="
done