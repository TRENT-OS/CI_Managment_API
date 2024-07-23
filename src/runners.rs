use rocket::serde::{Serialize, Deserialize, json::Json};
use std::env;
use reqwest::Client;
use rocket::http::Status;
use reqwest::header;
use rocket_db_pools::Connection;


use crate::db::RunnerDb;
use crate::db::runner_exists;

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    token: String,
}

async fn fetch_github_token(owner: &str, repo: &str, pat: &str) -> Result<TokenResponse, reqwest::Error> {
    println!("Beginning request");
    let client = Client::new();
    let url = format!("https://api.github.com/repos/{}/{}/actions/runners/registration-token", owner, repo);
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", pat))
        .header("Accept", "application/vnd.github.v3+json")
        .header(header::USER_AGENT, env!("CARGO_PKG_NAME"))
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;

    Ok(response)
}


async fn update_db_token_issue(runner_id: &str) {
    eprintln!("PLEASE IMPLEMENT ME :(\n Update token issue for {}", runner_id);
    return
}

async fn update_db_token_issue_failed(runner_id: &str) {
    eprintln!("PLEASE IMPLEMENT ME :(\n Update token issue FAILED for {}", runner_id);
    return
}


pub async fn runner_return_github_token(mut db: Connection<RunnerDb>, runner: &str) -> Result<Json<TokenResponse>, Status> {
    dotenv::dotenv().ok();
    let owner = env::var("GITHUB_OWNER").ok();
    let repo = env::var("GITHUB_REPO").ok();
    let pat = env::var("GITHUB_PAT").ok();

    if owner.is_none() || repo.is_none() || pat.is_none() {
        eprintln!("Missing required environment variables");
        return Err(Status::InternalServerError);
    }

    if !runner_exists(db, runner).await {
        eprintln!("Runner not found in database");
        return Err(Status::BadRequest);
    }

    let token = fetch_github_token(&owner.unwrap(), &repo.unwrap(), &pat.unwrap()).await;

    match token {
        Ok(token) => {
            update_db_token_issue(runner).await;
            Ok(Json(token))
        },
        Err(_) => {
            update_db_token_issue_failed(runner).await;
            Err(Status::InternalServerError)
        }
    }
}

