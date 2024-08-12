use chrono::{DateTime, Utc};
use std::{env, sync::LazyLock};
use rocket::serde::{Deserialize, Serialize};
use rocket_okapi::okapi::{schemars, schemars::JsonSchema};

#[derive(Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Timestamp {
    timestamp: i64,
    time_str: String,
}

static VALIDITY: LazyLock<i64> = LazyLock::new(|| {
    let input = env::var("RUNNER_VALIDITY").unwrap_or("60min".to_string());
    let validity = 60 * 60; // Default value 60 Minutes
    
    let ending = &input[input.len() - 3..];
    if let Ok(value) = &input[..input.len() - 3].parse::<i64>() {
        match ending {
            "sec" => *value,
            "min" => *value * 60,
            "hrs" => *value * 60 * 60,
            "day" => *value * 60 * 60 * 24,
            _ => validity,
        }
    } else {
        validity
    }
});


impl Timestamp {
    pub fn new() -> Self {
        let timestamp = Utc::now().timestamp() + *VALIDITY;
        Self {
            timestamp,
            time_str: DateTime::from_timestamp(timestamp, 0).expect("Invalid timestamp").to_rfc3339(),
        }
    }

    pub fn from(stmp: chrono::NaiveDateTime) -> Self {
        let dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(stmp, Utc);
        Self {
            timestamp: stmp.and_utc().timestamp(),
            time_str:  dt.to_rfc3339(),
        }
    }

    pub fn from_unix(timestamp: i64) -> Self {
        Self {
            timestamp,
            time_str: DateTime::from_timestamp(timestamp, 0).expect("Invalid timestamp").to_rfc3339()
        }
    }

    pub fn unix(&self) -> i64 {
        self.timestamp
    }

    pub fn to_string(&self) -> String {
        DateTime::from_timestamp(self.timestamp, 0).expect("Invalid timestamp").to_rfc3339()
    }

    pub fn chrono(&self) -> Option<chrono::NaiveDateTime> {
        if let Some(dt) = DateTime::from_timestamp(self.timestamp, 0) {
            Some(dt.naive_utc())
        } else {
            None
        }
    }
}
