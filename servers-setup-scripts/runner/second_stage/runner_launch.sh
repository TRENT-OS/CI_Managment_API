#!/bin/bash

IP="10.70.192.2"
PORT=8000

runner_id=$(tr -d '\n ' < /etc/runner_id)
url="http://$IP:$PORT/runner/$runner_id"
url_launch="$url/launch"

# Make a call to launch runner
status_code=$(curl -s -o /dev/null -w "%{http_code}" -X POST "$url_launch")
if [ "$status_code" -ne 200 ]; then
    echo "Failed: HTTP status code $status_code"
    exit 1 # Do not launch runner if the call fails
fi
