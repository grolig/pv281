use crate::models::Appointment;
use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait AppointmentRepo {
    async fn add_appointment(&self, patient_id: i32) -> anyhow::Result<u64>;
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
    async fn add_appointment(&self, patient_id: i32) -> anyhow::Result<u64> {
        let query_result =
            sqlx::query("INSERT INTO appointment (patient_id, start_time) VALUES ($1, $2)")
                .bind(patient_id)
                .bind(Utc::now())
                .execute(&*self.pg_pool)
                .await?;

        Ok(query_result.rows_affected())
    }

    async fn list_appointments(&self) -> anyhow::Result<Vec<Appointment>> {
        let appointments = sqlx::query_as!(
            Appointment,
            r#"SELECT id as "id!", start_time as "start_time!", last_name as "last_name!" FROM appointment NATURAL JOIN patient"#
        )
        .fetch_all(&*self.pg_pool)
        .await?;

        Ok(appointments)
    }
}
