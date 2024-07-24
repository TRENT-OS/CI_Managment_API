use rocket_db_pools::sqlx::SqliteConnection;
use rocket::{http::Status, serde::Serialize};
use serde::Deserialize;
use anyhow::Result;

use crate::db::{self, HardwareStatus};


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct HardwareInfo {
    pub name: String,
    pub status: db::HardwareStatus,
    pub claimed_by: Option<String>,
}

impl HardwareInfo {
    pub fn new(name: String, status: db::HardwareStatus, claimed_by: Option<String>) -> Self {
        Self {
            name,
            status,
            claimed_by,
        }
    }


    pub async fn retrieve(db: &mut SqliteConnection, hardware: &str) -> Option<Self> {
        if !db::hardware_exists(db, hardware).await {
            eprintln!("Hardware does not exist");
            return None;
        }

        Some(db::get_hardware_info(db, hardware).await)
    }
}


pub async fn is_hardware_available(db: &mut SqliteConnection, hardware: &str) -> Result<bool> {
    if !db::hardware_exists(db, hardware).await {
        return Err(anyhow::anyhow!("Hardware does not exist"));
    }
    return Ok(db::HardwareStatus::CLAIMED != db::get_hardware_status(db, hardware).await)
}

pub async fn hardware_info(db: &mut SqliteConnection) -> Vec<HardwareInfo> {   
    let boards = db::hardware_board_list(db).await;

    println!("Boards: {:?}", boards);


    let mut hardware_info: Vec<HardwareInfo> = Vec::new();
    for board in boards {
        hardware_info.push(
            if let Some(hwi) = HardwareInfo::retrieve(db, &board).await {
                println!("HWI: {:?}", hwi);
                hwi
            } else {
                eprintln!("Uhhh something went wrong :(");
                continue
            }
        )
    }

    hardware_info
}


pub async fn claim_hardware(db: &mut SqliteConnection, hardware: &str, runner: &str) -> Status {
    if let Ok(available) = is_hardware_available(db, hardware).await {
        if !available {
            return Status::Conflict;
        }        
    } else {
        return Status::NotFound;
    }

    match db::update_hardware_status(db, hardware, runner,  db::HardwareStatus::CLAIMED).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

pub async fn release_hardware(db: &mut SqliteConnection, hardware: &str, runner: &str) -> Status {
    if let Ok(available) = is_hardware_available(db, hardware).await {
        if available {
            return Status::Conflict;
        }        
    } else {
        return Status::NotFound;
    }
    
    match db::update_hardware_status(db, hardware, runner,  db::HardwareStatus::FREE).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

