use crate::repo::appointment_repo::{AppointmentRepo, PostgresAppointmentRepo};
use crate::repo::patient_repo::{PatientRepo, PostgresPatientRepo};
use crate::templates::Index;
use actix_web::{get, post, web, HttpResponse, Responder};
use askama::Template;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct PatientPostData {
    first_name: String,
    last_name: String,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct AppointmentPostData {
    patient_id: i32,
}

#[get("/patients")]
pub async fn get_patients(data: web::Data<Arc<PostgresPatientRepo>>) -> impl Responder {
    let patients = data.list_patients().await.unwrap_or_default();

    HttpResponse::Ok().json(patients)
}

#[post("/patient")]
pub async fn add_patient(
    data: web::Data<Arc<PostgresPatientRepo>>,
    body: web::Json<PatientPostData>,
) -> impl Responder {
    let result = data
        .add_patient(body.first_name.clone(), body.last_name.clone())
        .await;

    match result {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

#[get("/appointments")]
pub async fn get_appointments(data: web::Data<Arc<PostgresAppointmentRepo>>) -> impl Responder {
    let appointments = data.list_appointments().await.unwrap_or_default();

    HttpResponse::Ok().json(appointments)
}

#[post("/appointment")]
pub async fn add_appointment(
    data: web::Data<Arc<PostgresAppointmentRepo>>,
    body: web::Json<AppointmentPostData>,
) -> impl Responder {
    let result = data.add_appointment(body.patient_id).await;

    match result {
        Ok(_) => HttpResponse::Created(),
        Err(_) => HttpResponse::InternalServerError(),
    }
}

pub async fn index(
    data: web::Data<Arc<PostgresAppointmentRepo>>,
) -> actix_web::Result<HttpResponse> {
    let appointments = data.list_appointments().await.unwrap_or_default();
    let template = Index { appointments };

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(template.render().unwrap()))
}
