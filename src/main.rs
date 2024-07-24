mod runners;
mod db;
mod reset_task;
mod hardware;


#[macro_use] extern crate rocket;

use rocket::serde::json::Json;
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket_db_pools::Database;

// Runner Endpoint
#[get("/runner/info")]
async fn runner_info() -> &'static str {
    "Not Implemented"
}

#[get("/runner/<runner_id>/registration-token")]
async fn runner_registration_token(db: Connection<db::RunnerDb>, runner_id: &str) -> Result<Json<runners::TokenResponse>, Status> {
    runners::runner_return_github_token(db, runner_id).await
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
async fn hardware_info(mut db: Connection<db::RunnerDb>) -> Result<Json<Vec<hardware::HardwareInfo>>, Status> {
    Ok(Json(hardware::hardware_info(&mut db).await))
}

#[get("/hardware/<board_id>/info")]
async fn hardware_board_info(mut db: Connection<db::RunnerDb>, board_id: &str) -> Result<Json<hardware::HardwareInfo>, Status> {
    if let Some(hardware_info) = hardware::HardwareInfo::retrieve(&mut db, board_id).await {
        return Ok(Json(hardware_info));
    } else {
        return Err(Status::NotFound);
    }
}


#[get("/hardware/<board_id>/available")]
async fn hardware_board_available(mut db: Connection<db::RunnerDb>, board_id: &str) -> Result<String, Status> {
    if let Ok(available) = hardware::is_hardware_available(&mut db, board_id).await {
        return Ok(available.to_string());
    }
    Err(Status::NotFound)
}


#[get("/hardware/<board_id>/claim/<runner>")]
async fn hardware_board_claim(mut db: Connection<db::RunnerDb>, board_id: &str, runner: &str) -> Status {
    hardware::claim_hardware(&mut db, board_id, runner).await
}

#[get("/hardware/<board_id>/release/<runner>")]
async fn hardware_board_release(mut db: Connection<db::RunnerDb>, board_id: &str, runner: &str) -> Status {
    hardware::release_hardware(&mut db, board_id, runner).await
}


#[launch]
fn rocket() -> _ {

    //https://api.rocket.rs/v0.4/rocket_contrib/databases/#provided
    rocket::build()
        .attach(db::RunnerDb::init())
        .attach(reset_task::RunnerResetTask)
        .mount("/", routes![
               runner_info ,
               runner_registration_token,
               runner_launch,
               runner_reset,
               hardware_info,
               hardware_board_info,
               hardware_board_claim,
               hardware_board_available,
               hardware_board_release,
        ])
}
