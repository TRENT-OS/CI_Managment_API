# CI_Managment_API



## Sqlx Prepare 

```sh
DATABASE_URL="sqlite:$(pwd)/db/runner-managment-api.sqlite" cargo sqlx prepare
```


## Vars

```sh
GITHUB_OWNER="user"

GITHUB_REPO="repository"

GITHUB_PAT="github_pat_21321123213...."

RUNNER_VALIDITY="60min" # might be sec, min, hrs, day
```