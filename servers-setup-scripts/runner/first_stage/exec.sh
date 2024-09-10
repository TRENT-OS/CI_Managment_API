#!/bin/bash

# Make new snapshot to lighten update load
#			   days hours min secs
TIME_TO_RESNAP=$[ 30 * 24 * 60 * 60 ]   # = 30 days

IP="10.70.192.2"
PORT=8000

SNAPSHOT_STMP="/usr/local/libexec/setup_first_stage/last_snapshot"

# check if run with root privileges
if [ "$EUID" -ne 0 ]; then
	echo "Please run as root"
	exit 1
fi

runner_id=$(tr -d '\n ' < /etc/runner_id)

# check if file exists
if [ ! -f $SNAPSHOT_STMP ]; then
	echo 0 > $SNAPSHOT_STMP
fi
last_reset=$(tr -d '\n ' < $SNAPSHOT_STMP)


url="http://$IP:$PORT/runner/$runner_id"
url_snap="$url/vm/snapshot"

dnf upgrade -y

if (( $(date +%s) - last_reset > TIME_TO_RESNAP )); then
	status_code=$(curl -X POST -o /dev/null -w "%{http_code}" -s "$url_snap")
	if [ "$status_code" -ne 200 ]; then
		echo "Failed: HTTP status code $status_code"
		reboot
	fi

	date +%s > $SNAPSHOT_STMP

	# We just leave the script and idle...
	# -> snapshot will be created and the server will be reset.
	exit 0
fi

# enable the service, so that it is run after reboot
systemctl enable setup_second_stage.service

#disable this service, so it is not run again
systemctl disable setup_first_stage.service

#reboot in case a kernel update is installed
reboot
