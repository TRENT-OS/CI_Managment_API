use rocket::tokio::time::{interval, Duration};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Rocket, Build};



async fn reset_task() {
    let mut interval = interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        // Your cleanup logic here
        println!("Running cleanup task...");
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
        rocket::tokio::spawn(reset_task());
        Ok(rocket)
    }
}