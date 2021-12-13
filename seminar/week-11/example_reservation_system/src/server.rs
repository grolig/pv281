use std::env;
use std::sync::Arc;

use crate::handlers::{MyAppointmentService, MyPatientService};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tonic::transport::Server;

use crate::repo::appointment_repo::PostgresAppointmentRepo;
use crate::repo::patient_repo::PostgresPatientRepo;

mod handlers;
mod models;
mod repo;
mod templates;

pub mod patient {
    tonic::include_proto!("patient");
}

pub mod appointment {
    tonic::include_proto!("appointment");
}

use appointment::appointment_service_server::AppointmentServiceServer;
use patient::patient_service_server::PatientServiceServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = Arc::new(setup_pool().await?);

    let patient_repo = Arc::new(PostgresPatientRepo::new(pool.clone()));
    let appointment_repo = Arc::new(PostgresAppointmentRepo::new(pool.clone()));

    let address = "127.0.0.1:50051".parse()?;
    let patient_service = MyPatientService::new(patient_repo.clone());
    let appointment_service = MyAppointmentService::new(appointment_repo.clone());

    Server::builder()
        .add_service(PatientServiceServer::new(patient_service))
        .add_service(AppointmentServiceServer::new(appointment_service))
        .serve(address)
        .await?;

    Ok(())
}

async fn setup_pool() -> anyhow::Result<Pool<Postgres>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
