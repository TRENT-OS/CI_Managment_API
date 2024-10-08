//
// Copyright (C) 2024, HENSOLDT Cyber GmbH
// 
// SPDX-License-Identifier: GPL-2.0-or-later
//
// For commercial licensing, contact: info.cyber@hensoldt.net
//


mod db;
mod hardware;
mod reset_task;
mod runners;
mod timestamp;
mod vm;

#[macro_use]
extern crate rocket;

use rocket::{
    fairing::{self, AdHoc},
    http::Status,
    serde::json::Json,
    Build, Rocket,
};
use rocket_db_pools::{sqlx, Connection, Database};
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};



//------------------------------------------------------------------------------
// Runner
//------------------------------------------------------------------------------


#[openapi(tag = "Runner", ignore="db")]
#[get("/runner/info")]
async fn runners_info(mut db: Connection<db::RunnerDb>) -> Result<Json<Vec<runners::RunnerInfo>>, Status> {
    Ok(Json(runners::runners_info(&mut db).await))
}


#[openapi(tag = "Runner", ignore = "db")]
#[get("/runner/<runner_id>/info")]
async fn runner_info(mut db: Connection<db::RunnerDb>, runner_id: &str,) -> Result<Json<runners::RunnerInfo>, Status> {
    match runners::runner_info(&mut db, runner_id).await {
        Some(info) => Ok(Json(info)),
        None => Err(Status::NotFound),
    }
}


#[openapi(tag = "Runner", ignore = "db")]
#[get("/runner/<runner_id>/registration-token")]
async fn runner_registration_token(
    db: Connection<db::RunnerDb>,
    runner_id: &str,
) -> Result<String, Status> {
    runners::runner_return_github_token(db, runner_id).await
}


#[openapi(tag = "Runner", ignore = "db")]
#[post("/runner/<runner_id>/launch")]
async fn runner_launch(mut db: Connection<db::RunnerDb>, runner_id: &str) -> Status {
    runners::runner_launch(&mut db, runner_id).await
}


// VM Control functions
#[openapi(tag = "Runner", ignore = "db")]
#[post("/runner/<runner_id>/vm/reset")]
async fn runner_vm_reset(mut db: Connection<db::RunnerDb>, runner_id: &str) -> Status {
    runners::runner_reset(&mut db, runner_id).await
}


#[openapi(tag = "Runner", ignore = "db")]
#[post("/runner/<runner_id>/vm/snapshot")]
async fn runner_vm_snapshot(db: Connection<db::RunnerDb>, runner_id: &str) -> Status {
    runners::vm_snapshot(db, runner_id).await
}


#[openapi(tag = "Runner", ignore = "db")]
#[post("/runner/<runner_id>/vm/start")]
async fn runner_vm_start(mut db: Connection<db::RunnerDb>, runner_id: &str) -> Status {
    runners::vm_start(&mut db, &runner_id).await
}


#[openapi(tag = "Runner", ignore = "db")]
#[post("/runner/<runner_id>/vm/stop")]
async fn runner_vm_stop(mut db: Connection<db::RunnerDb>, runner_id: &str) -> Status {
    runners::vm_stop(&mut db, runner_id).await
}


//------------------------------------------------------------------------------
// Hardware
//------------------------------------------------------------------------------


#[openapi(tag = "Hardware", ignore = "db")]
#[get("/hardware/info")]
async fn hardware_info(
    mut db: Connection<db::RunnerDb>,
) -> Result<Json<Vec<hardware::HardwareInfo>>, Status> {
    Ok(Json(hardware::hardware_info(&mut db).await))
}


#[openapi(tag = "Hardware", ignore = "db")]
#[get("/hardware/<board_id>/info")]
async fn hardware_board_info(
    mut db: Connection<db::RunnerDb>,
    board_id: &str,
) -> Result<Json<hardware::HardwareInfo>, Status> {
    if let Some(hardware_info) = hardware::HardwareInfo::retrieve(&mut db, board_id).await {
        return Ok(Json(hardware_info));
    } else {
        return Err(Status::NotFound);
    }
}


#[openapi(tag = "Hardware", ignore = "db")]
#[get("/hardware/<board_id>/available")]
async fn hardware_board_available(
    mut db: Connection<db::RunnerDb>,
    board_id: &str,
) -> Result<String, Status> {
    if let Ok(available) = hardware::is_hardware_available(&mut db, board_id).await {
        return Ok(available.to_string());
    }
    Err(Status::NotFound)
}


#[openapi(tag = "Hardware", ignore = "db")]
#[post("/hardware/<board_id>/claim/<runner>")]
async fn hardware_board_claim(
    mut db: Connection<db::RunnerDb>,
    board_id: &str,
    runner: &str,
) -> Status {
    return hardware::claim_hardware(&mut db, board_id, runner)
        .await
        .unwrap_or(Status::InternalServerError);
}


#[openapi(tag = "Hardware", ignore = "db")]
#[post("/hardware/<board_id>/release/<runner>")]
async fn hardware_board_release(
    mut db: Connection<db::RunnerDb>,
    board_id: &str,
    runner: &str,
) -> Status {
    return hardware::release_hardware(&mut db, board_id, runner)
        .await
        .unwrap_or(Status::InternalServerError);
}



//------------------------------------------------------------------------------
// Sqlx Migrations
//------------------------------------------------------------------------------


async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match db::RunnerDb::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}



//------------------------------------------------------------------------------
// Launch Rocketttt blazzziinngglyyy fast 🚀🚀🚀
//------------------------------------------------------------------------------


#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(db::RunnerDb::init())
        .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
        .attach(reset_task::RunnerResetTask)
        .mount(
            "/",
            openapi_get_routes![
                runner_info,
                runners_info,
                runner_registration_token,
                runner_launch,
                runner_vm_reset,
                runner_vm_snapshot,
                runner_vm_start,
                runner_vm_stop,
                hardware_info,
                hardware_board_info,
                hardware_board_claim,
                hardware_board_available,
                hardware_board_release,
            ],
        )
        .mount(
            "/docs/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
