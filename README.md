# CI_Managment_API

This API is part of the TRENTOS Hardware CI setup. 

It supports the following features:
 - issuance of One-Time-Tokens for self-hosted Runner Authentication
 - Resetting of VMs via SMB shares
    - Force resetting after a time threshold
 - Hardware allocation via a sqlite database


## Sqlx Prepare 
```sh
DATABASE_URL="sqlite:$(pwd)/db/runner-managment-api.sqlite" cargo sqlx prepare
```

## Vars

These can be provided via the environment or via a .env file:
```sh
GITHUB_OWNER="user"

GITHUB_REPO="repository"

GITHUB_PAT="github_pat_21321123213...."

RUNNER_VALIDITY="60min" # might be sec, min, hrs, day

PROXY_URL="http://my.proxy:8080"
```