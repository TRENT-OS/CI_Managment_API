use rocket::tokio::time::{interval, Duration};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Rocket, Build};
use rocket_db_pools::Database;
use rocket_db_pools::sqlx::pool::PoolConnection;
use rocket_db_pools::sqlx::Sqlite;
use rocket_db_pools::sqlx;


use crate::db::RunnerDb;

async fn reset_task(mut db: PoolConnection<Sqlite>) {
    let mut interval = interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        // Your cleanup logic here
        
        let data = sqlx::query!("SELECT Id, Status, ClaimedBy FROM Hardware").fetch_all(&mut *db).await.unwrap();

        for rec in data {
            println!(
                "- {} {} {:?}",
                rec.Id,
                rec.Status,
                rec.ClaimedBy,
            );
        }


        println!("Resetting task");
    }
}

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
        let db_pool = match RunnerDb::fetch(&rocket) {
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