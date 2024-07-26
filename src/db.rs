use anyhow::Result;
use rocket::serde::Serialize;
use rocket_db_pools::sqlx::{self, SqliteConnection};
use rocket_db_pools::Database;
use serde::Deserialize;
use std::str::FromStr;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;


use strum_macros::{AsRefStr, EnumString};

use crate::{hardware, runners, timestamp};

//------------------------------------------------------------------------------
// Data Structures
//------------------------------------------------------------------------------


#[derive(Database)]
#[database("runner_db")]
pub struct RunnerDb(sqlx::SqlitePool);

#[derive(Debug, AsRefStr, PartialEq, EnumString, Serialize, Deserialize, JsonSchema)]
pub enum RunnerStatus {
    RESETTING,
    IDLE,
    RUNNING,
    ERROR,
    OFFLINE,
}

#[derive(Debug, AsRefStr, PartialEq, EnumString, Serialize, Deserialize, JsonSchema)]
pub enum HardwareStatus {
    FREE,
    CLAIMED,
    UNAVAILABLE,
    ERROR,
}



//------------------------------------------------------------------------------
// Runner
//------------------------------------------------------------------------------


pub async fn runner_exists(db: &mut SqliteConnection, runner: &str) -> bool {
    1 == sqlx::query_as::<_, (i64,)>("SELECT EXISTS (SELECT 1 FROM RunnerVMs WHERE Id = ?)")
        .bind(runner)
        .fetch_one(db)
        .await
        .unwrap()
        .0
}

pub async fn update_runner_status(db: &mut SqliteConnection, runner: &str, status: RunnerStatus) {
    sqlx::query("UPDATE RunnerVMs SET Status = ? WHERE Id = ?")
        .bind(status.as_ref())
        .bind(runner)
        .execute(db)
        .await
        .unwrap();
}

pub async fn update_runner_time_to_reset(
    db: &mut SqliteConnection,
    runner: &str,
    time_to_reset: Option<chrono::NaiveDateTime>,
) {
    sqlx::query("UPDATE RunnerVMs SET TimeToReset = ? WHERE Id = ?")
        .bind(time_to_reset)
        .bind(runner)
        .execute(db)
        .await
        .unwrap();
}


pub async fn runner_id_list(db: &mut SqliteConnection) -> Vec<String> {
    sqlx::query!("SELECT Id FROM RunnerVMs")
        .fetch_all(&mut *db)
        .await
        .unwrap()
        .into_iter()
        .map(|rec| rec.Id)
        .collect()
}


pub async fn get_runner_info(
    db: &mut SqliteConnection,
    runner: &str,
) -> runners::RunnerInfo {
    let data = sqlx::query!(
        "SELECT Id, Status, TimeToReset FROM RunnerVMs WHERE Id = ?",
        runner
    )
    .fetch_one(db)
    .await
    .unwrap();

    let runner_status = RunnerStatus::from_str(&data.Status)
        .expect("Invalid Hardware Status: Database Corruption");
    let timestamp = if let Some(t) = data.TimeToReset {
        Some(timestamp::Timestamp::from(t))
    } else {
        None
    };
    runners::RunnerInfo::new(data.Id, runner_status, timestamp)
}




//------------------------------------------------------------------------------
// Hardware
//------------------------------------------------------------------------------


pub async fn hardware_exists(db: &mut SqliteConnection, hardware: &str) -> bool {
    1 == sqlx::query_as::<_, (i64,)>("SELECT EXISTS (SELECT 1 FROM Hardware WHERE Id = ?)")
        .bind(hardware)
        .fetch_one(db)
        .await
        .unwrap()
        .0
}

pub async fn get_hardware_status(db: &mut SqliteConnection, hardware: &str) -> HardwareStatus {
    let a = sqlx::query!("SELECT Status FROM Hardware WHERE Id = ?", hardware)
        .fetch_one(db)
        .await
        .unwrap()
        .Status;
    HardwareStatus::from_str(&a).expect("Invalid HardwareStatus in database: Database corruption")
}

pub async fn get_hardware_claimed_by_runner(
    db: &mut SqliteConnection,
    runner: &str,
) -> Vec<String> {
    sqlx::query!(
        "SELECT Id FROM Hardware WHERE ClaimedBy = ?",
        runner
    )
    .fetch_all(db)
    .await
    .unwrap()
    .into_iter()
    .map(|rec| rec.Id)
    .collect()
}

pub async fn update_hardware_status(
    db: &mut SqliteConnection,
    hardware: &str,
    runner: &str,
    status: HardwareStatus,
) -> Result<()> {
    let status_str = status.as_ref().to_owned();
    sqlx::query!(
        "UPDATE Hardware SET Status = ?, ClaimedBy = ? WHERE Id = ?",
        status_str,
        runner,
        hardware
    )
    .execute(db)
    .await
    .unwrap();
    Ok(())
}

pub async fn get_hardware_info(
    db: &mut SqliteConnection,
    hardware: &str,
) -> hardware::HardwareInfo {
    let data = sqlx::query!(
        "SELECT Id, Status, ClaimedBy FROM Hardware WHERE Id = ?",
        hardware
    )
    .fetch_one(db)
    .await
    .unwrap();

    let hw_status = HardwareStatus::from_str(&data.Status)
        .expect("Invalid Hardware Status: Database Corruption");
    hardware::HardwareInfo::new(data.Id, hw_status, data.ClaimedBy)
}

pub async fn hardware_board_list(db: &mut SqliteConnection) -> Vec<String> {
    sqlx::query!("SELECT Id FROM Hardware")
        .fetch_all(db)
        .await
        .unwrap()
        .into_iter()
        .map(|rec| rec.Id)
        .collect()
}
