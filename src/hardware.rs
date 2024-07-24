use rocket_db_pools::sqlx::SqliteConnection;


use crate::db::{get_hardware_status, HardwareStatus};


pub async fn is_hardware_claimed(db: &mut SqliteConnection, hardware: &str) -> bool {
    HardwareStatus::CLAIMED == get_hardware_status(db, hardware).await
}