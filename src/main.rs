mod runners;
mod db;


#[macro_use] extern crate rocket;

use rocket::serde::{json::Json};
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket_db_pools::Database;

use crate::db::RunnerDb;
use crate::runners::{TokenResponse, runner_return_github_token};

// Runner Endpoint
#[get("/runner/info")]
async fn runner_info() -> &'static str {
    "Not Implemented"
}

#[get("/runner/<runner_id>/registration-token")]
async fn runner_registration_token(mut db: Connection<RunnerDb>, runner_id: &str) -> Result<Json<TokenResponse>, Status> {
    return runner_return_github_token(db, runner_id).await
}

#[post("/runner/<runner_id>/launch")]
async fn runner_launch(runner_id: &str) -> &'static str {
    "Not Implemented"
}

#[post("/runner/<runner_id>/reset")]
async fn runner_reset(runner_id: &str) -> &'static str {
    "Not Implemented"
}


// Hardware Endpoint
#[get("/hardware/info")]
async fn hardware_info() -> &'static str {
    "Not Implemented"
}

#[get("/hardware/<board_id>/status")]
async fn hardware_board_status(board_id: &str) -> &'static str {
    "Not Implemented"
}

#[get("/runner/<board_id>/claim")]
async fn hardware_board_claim(board_id: &str) -> &'static str {
    "Not Implemented"
}

#[get("/runner/<board_id>/release")]
async fn hardware_board_release(board_id: &str) -> &'static str {
    "Not Implemented"
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(RunnerDb::init())
        .mount("/", routes![
               runner_info ,
               runner_registration_token,
               runner_launch,
               runner_reset,
               hardware_info,
               hardware_board_status,
               hardware_board_claim,
               hardware_board_release,
        ])
}
