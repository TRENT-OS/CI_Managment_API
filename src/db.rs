use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self};

#[derive(Database)]
#[database("runner_db")]
pub struct RunnerDb(sqlx::SqlitePool);


pub async fn runner_exists(mut db: Connection<RunnerDb>, runner: &str) -> bool  {
    let exists: (i64,) = sqlx::query_as("SELECT EXISTS (SELECT 1 FROM RunnerVMs WHERE Name = ?)")
        .bind(runner)
        .fetch_one(&mut **db)
        .await.unwrap();

    1 == exists.0
}

pub async fn hardware_exists(mut db: Connection<RunnerDb>, hardware: &str) -> bool  {
    let query = format!("SELECT EXISTS (SELECT 1 FROM RunnerVMs WHERE Name = '{}'", hardware);

    let exists: (i64,) = sqlx::query_as("SELECT EXISTS (SELECT 1 FROM RunnerVMs WHERE Name = ?)")
        .bind(hardware)
        .fetch_one(&mut **db)
        .await.unwrap();

    1 == exists.0
}
