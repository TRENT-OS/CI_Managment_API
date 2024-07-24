use rocket_db_pools::Database;
use rocket_db_pools::sqlx::{self, SqliteConnection};

use std::str::FromStr;


use strum_macros::{AsRefStr, EnumString};    


#[derive(Database)]
#[database("runner_db")]
pub struct RunnerDb(sqlx::SqlitePool);


#[derive(Debug, AsRefStr, PartialEq, EnumString)]
pub enum RunnerStatus {
    RESETTING,
    IDLE,
    RUNNING,
    ERROR,
    OFFLINE,
}


#[derive(Debug, AsRefStr, PartialEq, EnumString)]
pub enum HardwareStatus {
    FREE,
    CLAIMED,
    UNAVAILABLE,
}


// Runner
pub async fn runner_exists(db: &mut SqliteConnection, runner: &str) -> bool  {
    1 == sqlx::query_as::<_, (i64,)>("SELECT EXISTS (SELECT 1 FROM RunnerVMs WHERE Name = ?)")
       .bind(runner)
       .fetch_one(db)
       .await.unwrap()
       .0
}

pub async fn update_runner_status(db: &mut SqliteConnection, runner: &str, status: RunnerStatus) {
    sqlx::query("UPDATE RunnerVMs SET Status = ? WHERE Name = ?")
        .bind(status.as_ref())
        .bind(runner)
        .execute(db)
        .await.unwrap();
}


// Hardware
pub async fn hardware_exists(db: &mut SqliteConnection, hardware: &str) -> bool  {
    1 == sqlx::query_as::<_, (i64,)>("SELECT EXISTS (SELECT 1 FROM RunnerVMs WHERE Name = ?)")
        .bind(hardware)
        .fetch_one(db)
        .await.unwrap()
        .0
}

pub async fn get_hardware_status(db: &mut SqliteConnection, hardware: &str) -> HardwareStatus {
    let a = sqlx::query!("SELECT Status FROM Hardware WHERE Name = ?", hardware)
        .fetch_one(db)
        .await.unwrap()
        .Status;
    HardwareStatus::from_str(&a).expect("Invalid HardwareStatus in database: Database corruption")
}

pub async fn update_hardware_status(db: &mut SqliteConnection, hardware: &str, status: RunnerStatus) {
    sqlx::query("UPDATE Hardware SET Status = ? WHERE Name = ?")
        .bind(status.as_ref())
        .bind(hardware)
        .execute(db)
        .await.unwrap();
}
