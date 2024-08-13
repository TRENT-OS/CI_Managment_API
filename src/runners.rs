use reqwest::{header, Client, Proxy};
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::{Connection, sqlx::SqliteConnection};

use rocket_okapi::okapi::{schemars, schemars::JsonSchema};
use std::env;


use crate::hardware;
use crate::{db, timestamp, vm};


//------------------------------------------------------------------------------
// Data Structures
//------------------------------------------------------------------------------


#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RunnerInfo {
    pub name: String,
    pub status: db::RunnerStatus,
    pub time_to_reset: Option<timestamp::Timestamp>,
}

impl RunnerInfo {
    pub fn new(name: String, status: db::RunnerStatus, time_to_reset: Option<timestamp::Timestamp>) -> Self {
        Self {
            name,
            status,
            time_to_reset,
        }
    }

    pub async fn retrieve(db: &mut SqliteConnection, runner: &str) -> Option<Self> {
        if !db::runner_exists(db, runner).await {
            eprintln!("Runner does not exist");
            return None;
        }

        Some(db::get_runner_info(db, runner).await)
    }
}


#[derive(Serialize, Deserialize, JsonSchema)]
pub struct TokenResponse {
    token: String,
}

//------------------------------------------------------------------------------
// Runner Endpoint Logic
//------------------------------------------------------------------------------


pub async fn runner_info(db: &mut SqliteConnection, runner: &str) -> Option<RunnerInfo> {
    RunnerInfo::retrieve(db, runner).await
} 

pub async fn runners_info(mut db: &mut SqliteConnection) -> Vec<RunnerInfo> {
    let runners = db::runner_id_list(&mut db).await;


    let mut runner_info: Vec<RunnerInfo> = Vec::new();
    for runner in runners {
        let info = RunnerInfo::retrieve(&mut db, &runner).await;
        if let Some(info) = info {
            runner_info.push(info);
        }
    }
    runner_info
}

pub async fn runner_launch(db: &mut SqliteConnection, runner: &str) -> Status {
    if !db::runner_exists(db, runner).await {
        eprintln!("Runner does not exist");
        return Status::NotFound;
    }

    let timestamp = timestamp::Timestamp::new().chrono();
    db::update_runner_time_to_reset(db, runner, timestamp).await;
    db::update_runner_status(db, runner, db::RunnerStatus::IDLE).await;

    println!("Launching runner {}", runner);
    Status::Ok
}


async fn release_hardware(db: &mut SqliteConnection, runner: &str) {
    let claimed_hw = db::get_hardware_claimed_by_runner(db, runner).await;

    for hardware in claimed_hw {
        let _ = hardware::release_hardware(db, &hardware, runner).await; // Ignore result
    }
}


pub async fn runner_reset(db: &mut SqliteConnection, runner: &str) -> Status {
    if !db::runner_exists(db, runner).await {
        eprintln!("Runner does not exist");
        return Status::NotFound;
    }

    let timestamp = None;
    db::update_runner_time_to_reset(db, runner, timestamp).await;
    db::update_runner_status(db, runner, db::RunnerStatus::RESETTING).await;

    release_hardware(db, runner).await; // release all hardware claimed by runner
    vm::reset(runner).await;

    println!("Resetting runner {}", runner);
    Status::Ok
}


async fn fetch_github_token(
    owner: &str,
    repo: &str,
    pat: &str,
    proxy: &str,
) -> Result<TokenResponse, reqwest::Error> {
    println!("Beginning request");

    let client = Client::builder()
        .proxy(Proxy::https(proxy)?)
        .build()?;

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
    mut db: Connection<db::RunnerDb>,
    runner: &str,
) -> Result<Json<TokenResponse>, Status> {
    dotenv::dotenv().ok();
    let owner = env::var("GITHUB_OWNER").ok();
    let repo = env::var("GITHUB_REPO").ok();
    let pat = env::var("GITHUB_PAT").ok();
    let proxy = env::var("PROXY_URL").ok();

    if owner.is_none() || repo.is_none() || pat.is_none() || proxy.is_none() {
        eprintln!("Missing required environment variables");
        return Err(Status::InternalServerError);
    }

    if !db::runner_exists(&mut db, runner).await {
        eprintln!("Runner not found in database");
        return Err(Status::BadRequest);
    }

    let token = fetch_github_token(&owner.unwrap(), &repo.unwrap(), &pat.unwrap(), &proxy.unwrap()).await;

    match token {
        Ok(token) => {
            db::update_runner_status(&mut db, runner, db::RunnerStatus::IDLE).await;
            Ok(Json(token))
        }
        Err(_) => {
            db::update_runner_status(&mut db, runner, db::RunnerStatus::ERROR).await;
            Err(Status::InternalServerError)
        }
    }
}


pub async fn vm_snapshot(mut db: Connection<db::RunnerDb>, runner: &str) -> Status {
    if !db::runner_exists(&mut db, runner).await {
        eprintln!("Runner does not exist");
        return Status::NotFound;
    }

    let runner = runner.to_string(); // required due to spawn closure

    rocket::tokio::spawn(async move {
        db::update_runner_status(&mut db, &runner, db::RunnerStatus::RESETTING).await;

        vm::snapshot(&runner).await;
        println!("Snapshotting runner {}", runner);

        std::thread::sleep(std::time::Duration::from_secs(20));
    
        runner_reset(&mut db, &runner).await;
    });

    return Status::Ok;
}


pub async fn vm_start(db: &mut SqliteConnection, runner: &str) -> Status {
    if !db::runner_exists(db, runner).await {
        eprintln!("Runner does not exist");
        return Status::NotFound;
    }

    db::update_runner_status(db, runner, db::RunnerStatus::RUNNING).await;
    vm::start(runner).await;
    println!("Starting runner vm {}", runner);

    Status::Ok
}


pub async fn vm_stop(db: &mut SqliteConnection, runner: &str) -> Status {
    if !db::runner_exists(db, runner).await {
        eprintln!("Runner does not exist");
        return Status::NotFound;
    }

    db::update_runner_status(db, runner, db::RunnerStatus::OFFLINE).await;
    vm::stop(runner).await;
    println!("Stopping runner vm {}", runner);

    Status::Ok
}