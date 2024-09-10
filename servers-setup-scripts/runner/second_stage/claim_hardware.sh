#!/bin/bash
dot_sequence=('.' '..' '...')
dot_index=0
dt_i=0


IP="10.70.192.2"
PORT=8000

if [ -z "$1" ]; then
  echo "Usage: $0 <board>"
  exit 1
fi

runner_id=$(tr -d '\n ' < /tmp/runner_id)

# URL to send the curl request to
url="http://$IP:$PORT/hardware/$1"

url_available="$url/available"
url_claim="$url/claim/$runner_id"

while true; do
	response=$(curl -s -w "%{http_code}" "$url_available")
    status_code="${response: -3}"
    response_body="${response::-3}"

	# Hardware is free
	if [ "$status_code" == "200" ] && [ "$response_body" == "true" ]; then
		#claim it
		claim_code=$(curl -X POST -s -o /dev/null -w "%{http_code}" "$url_claim")
		if [ "$claim_code" -ne 200 ]; then
			echo "Failed: HTTP status code $claim_code"
			exit 1
		fi

		if [ "$dt_i" -ne 0 ]; then
			echo -ne "\n"
		fi
		echo "Claimed hardware: $1"
        exit 0
    fi

	if [ "$status_code" != "200" ]; then
        echo "Failed: HTTP status code $status_code"
        exit 1
    fi

	for i in {1..5}; do
    	echo -ne "\rHardware $1 is not available, waiting${dot_sequence[$dot_index]}   "
		dot_index=$(( (dot_index + 1) % 3 ))
		sleep 1
	done

	#Loop till force reset or hardware is available
	dt_i=$(( dt_i + 5 ))
done