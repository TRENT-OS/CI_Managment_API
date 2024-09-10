#!/bin/bash

# script expects $IP & $PORT to be provided by the environment
IP="10.70.192.2"
PORT=8000

runner_id=$(tr -d '\n ' < /tmp/runner_id)
url="http://$IP:$PORT/runner/$runner_id"
url_reset="$url/vm/reset"

# Make a call to reset runner (reset also releases hardware)

$(curl -X POST -s -o /dev/null -w "%{http_code}" "$url_reset")
if [ "$status_code" -ne 200 ]; then
        echo "Failed: HTTP status code $status_code"
        exit 1 # We will wait for a force reset
fi
echo "Reset runner: $runner_id"
