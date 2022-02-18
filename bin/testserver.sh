#!/usr/bin/env bash
set -eo pipefail

# Default options
SERVER_PORT=8088
SERVER_IP_ADDR=192.168.0.1
ENABLE_NGROK=0

# Where ip addr and port are stored for firmware
PORT_FILE=target/HOST_PORT
IP_ADDR_FILE=target/HOST_IP_ADDR

while [ -n "$1" ]; do
    case "$1" in
        --ngrok)
            ENABLE_NGROK=1
        ;;
        --ip)
            SERVER_IP_ADDR=$2
            shift
        ;;
        --port)
            SERVER_PORT=$2
            shift
        ;;
    esac
    shift
done


mkdir -p target
rm $PORT_FILE 2&>1 > /dev/null || true
rm $IP_ADDR_FILE 2&>1 > /dev/null || true

if [ "$ENABLE_NGROK" -eq 1 ]; then
    echo "Setting up TCP tunnel with ngrok (run me again if I stop here...)"
    ngrok tcp $LOCAL_PORT 2>&1 > /dev/null || true &

    URL=$(curl -s http://localhost:4040/api/tunnels | jq -r '.tunnels[0].public_url')
    PARTS=($(basename $URL | tr ":" " "))
    DOMAIN=${PARTS[0]}
    SERVER_PORT=${PARTS[1]}
    SERVER_IP_ADDR=$(dig +short $DOMAIN @resolver1.opendns.com)
fi

echo -n $SERVER_PORT > $PORT_FILE
echo -n $SERVER_IP_ADDR > $IP_ADDR_FILE

echo "Server available at: $SERVER_IP_ADDR:$SERVER_PORT (${DOMAIN:-local})"
echo "Listening on local port $SERVER_PORT..."
while [ true ]
do
    
    echo "===== START TCP INPUT ====="
    nc -l 0.0.0.0 $SERVER_PORT
    echo "===== CONNECTION CLOSED ====="
done