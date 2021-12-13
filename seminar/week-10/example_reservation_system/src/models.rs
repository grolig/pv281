use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Appointment {
    pub id: i32,
    #[serde(with = "ts_milliseconds")]
    pub start_time: DateTime<Utc>,
    pub last_name: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Patient {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}
