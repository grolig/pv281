use crate::models::Appointment;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait AppointmentRepo {
    async fn add_appointment(&self, patient_id: i32) -> anyhow::Result<()>;
    async fn list_appointments(&self) -> anyhow::Result<Vec<Appointment>>;
}

pub struct PostgresAppointmentRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresAppointmentRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool }
    }
}

#[async_trait]
impl AppointmentRepo for PostgresAppointmentRepo {
    async fn add_appointment(&self, patient_id: i32) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO appointment (patient_id, start_time) VALUES ($1, $2)")
            .bind(patient_id)
            .bind(Utc::now())
            .execute(&*self.pg_pool)
            .await?;

        Ok(())
    }

    async fn list_appointments(&self) -> anyhow::Result<Vec<Appointment>> {
        let appointments = sqlx::query_as!(
            Appointment,
            // Here, I decided to get rid of Options which are returned from query by using "!"
            // (since we defined columns to be not null).
            // More about Options and nullability of columns 
            // can be found here https://docs.rs/sqlx/0.5.9/sqlx/macro.query.html.
            r#"SELECT id as "id!", start_time as "start_time!", last_name as "last_name!" FROM appointment NATURAL JOIN patient"#
        )
        .fetch_all(&*self.pg_pool)
        .await?;

        Ok(appointments)
    }
}
