use rocket::{
    fairing::{Fairing, Info, Kind},
    tokio::time::{interval, Duration},
    Build, Rocket,
};
use rocket_db_pools::{
    sqlx::{pool::PoolConnection, Sqlite},
    Database,
};
use chrono::Utc;

use crate::{db, runners};

//------------------------------------------------------------------------------
// Reset Logic
//------------------------------------------------------------------------------


async fn reset_task(mut db: PoolConnection<Sqlite>) {
    let mut interval = interval(Duration::from_secs(30));
    loop {
        interval.tick().await;

        let runners = runners::runners_info(&mut *db).await;

        for runner in runners {
            if let Some(time_to_reset) = runner.time_to_reset {
                if time_to_reset.unix() < Utc::now().timestamp() {
                    runners::runner_reset(&mut *db, &runner.name).await;
                    println!("Force reset of {}", runner.name);
                }
            }
        }
    }
}



//------------------------------------------------------------------------------
// Fairing Setup
//------------------------------------------------------------------------------


pub struct RunnerResetTask;

#[rocket::async_trait]
impl Fairing for RunnerResetTask {
    fn info(&self) -> Info {
        Info {
            name: "Reset Task",
            kind: Kind::Ignite | Kind::Liftoff,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let db_pool = match db::RunnerDb::fetch(&rocket) {
            Some(pool) => pool,
            None => {
                eprintln!("Failed to fetch the database pool");
                return Err(rocket);
            }
        };
        rocket::tokio::spawn(reset_task(db_pool.acquire().await.unwrap()));
        Ok(rocket)
    }
}
