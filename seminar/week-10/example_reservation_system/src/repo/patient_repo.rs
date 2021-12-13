use crate::models::Patient;
use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

#[async_trait]
pub trait PatientRepo {
    async fn add_patient(&self, name: String, surname: String) -> anyhow::Result<()>;
    async fn list_patients(&self) -> anyhow::Result<Vec<Patient>>;
}

pub struct PostgresPatientRepo {
    pg_pool: Arc<PgPool>,
}

impl PostgresPatientRepo {
    pub fn new(pg_pool: Arc<PgPool>) -> Self {
        Self { pg_pool }
    }
}

#[async_trait]
impl PatientRepo for PostgresPatientRepo {
    async fn add_patient(&self, first_name: String, last_name: String) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO patient (first_name, last_name) VALUES ($1, $2)")
            .bind(first_name)
            .bind(last_name)
            .execute(&*self.pg_pool)
            .await?;

        Ok(())
    }

    async fn list_patients(&self) -> anyhow::Result<Vec<Patient>> {
        let patients = sqlx::query_as!(Patient, "SELECT * FROM patient")
            .fetch_all(&*self.pg_pool)
            .await?;

        Ok(patients)
    }
}
