use std::env;
use std::sync::Arc;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

use crate::repo::appointment_repo::PostgresAppointmentRepo;
use crate::repo::patient_repo::PostgresPatientRepo;

mod handlers;
mod models;
mod repo;
mod templates;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let pool = Arc::new(setup_pool().await?);

    let patient_repo = Arc::new(PostgresPatientRepo::new(pool.clone()));
    let appointment_repo = Arc::new(PostgresAppointmentRepo::new(pool.clone()));

    // In order to display some appointments in the html template, you can add some patients/appointments
    // via Postman

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(patient_repo.clone()))
            .app_data(web::Data::new(appointment_repo.clone()))
            .service(handlers::get_patients)
            .service(handlers::get_appointments)
            .service(handlers::add_patient)
            .service(handlers::add_appointment)
            .service(web::resource("/").route(web::get().to(handlers::index)))
            // an example of how to serve static files (css in this case, just for a demonstration)
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:9000")?
    .run()
    .await?;

    Ok(())
}

async fn setup_pool() -> anyhow::Result<Pool<Postgres>> {
    // make sure to create database <appointment> at first (via adminer or sqlx-cli
    // `sqlx database create`), then apply the migration (again, via sqlx-cli `sqlx migrate run`)
    // we removed the macro `migrate!` since we use macro `query_as!` that connects to db
    // during compilation to check whether a query is correct. In order to use that macro,
    // the migration needs to be applied before, so that db is set up. For that reason,
    // we will do it manually this time
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
