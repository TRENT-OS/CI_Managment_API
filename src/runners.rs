use reqwest::header;
use reqwest::Client;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::Connection;
use std::borrow::BorrowMut;
use std::env;

use crate::db::{runner_exists, update_runner_status, RunnerDb, RunnerStatus};

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    token: String,
}

async fn fetch_github_token(
    owner: &str,
    repo: &str,
    pat: &str,
) -> Result<TokenResponse, reqwest::Error> {
    println!("Beginning request");
    let client = Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/actions/runners/registration-token",
        owner, repo
    );
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

pub async fn runner_return_github_token(
    mut db: Connection<RunnerDb>,
    runner: &str,
) -> Result<Json<TokenResponse>, Status> {
    dotenv::dotenv().ok();
    let owner = env::var("GITHUB_OWNER").ok();
    let repo = env::var("GITHUB_REPO").ok();
    let pat = env::var("GITHUB_PAT").ok();

    if owner.is_none() || repo.is_none() || pat.is_none() {
        eprintln!("Missing required environment variables");
        return Err(Status::InternalServerError);
    }

    if !runner_exists(&mut db, runner).await {
        eprintln!("Runner not found in database");
        return Err(Status::BadRequest);
    }

    let token = fetch_github_token(&owner.unwrap(), &repo.unwrap(), &pat.unwrap()).await;

    match token {
        Ok(token) => {
            update_runner_status(&mut db, runner, RunnerStatus::IDLE).await;
            Ok(Json(token))
        }
        Err(_) => {
            update_runner_status(&mut db, runner, RunnerStatus::ERROR).await;
            Err(Status::InternalServerError)
        }
    }
}
