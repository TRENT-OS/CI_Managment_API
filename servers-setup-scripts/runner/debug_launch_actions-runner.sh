#!/bin/bash


IP="10.70.192.2"
PORT=8000

if [ ! -f "/etc/runner_id" ]; then
    echo "Failure: Runner ID file not found under /etc/runner_id"
    exit 1
fi
runner_id=$(tr -d '\n ' < /etc/runner_id)

# retrieve one time token from tokenserver
token=$(curl -s http://$IP:$PORT/runner/$runner_id/registration-token | tr -d '"\n')

cd /home/actions-service-user/actions-runner

# Set pre and post job script environment variables
export ACTIONS_RUNNER_HOOK_JOB_STARTED="/usr/local/libexec/setup_second_stage/runner_launch.sh"
export ACTIONS_RUNNER_HOOK_JOB_COMPLETED="/usr/local/libexec/setup_second_stage/runner_reset.sh"
export https_proxy='http://proxy.cc.ebs.corp:8080'
export http_proxy='http://proxy.cc.ebs.corp:8080'
export SSL_CERT_DIR='/etc/ssl/certs'

./config.sh --url https://github.com/TRENT-OS --token $token --name $runner_id --unattended --replace --disableupdate --check
./run.sh