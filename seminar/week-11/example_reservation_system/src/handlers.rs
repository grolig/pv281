use crate::repo::appointment_repo::{AppointmentRepo, PostgresAppointmentRepo};
use crate::repo::patient_repo::{PatientRepo, PostgresPatientRepo};
use chrono::{DateTime, Utc};
use prost_types::Timestamp;
use std::sync::Arc;
use std::time::SystemTime;
use tonic::{Request, Response, Status};

use crate::appointment;
use crate::appointment::appointment_service_server::AppointmentService;
use crate::appointment::{AppointmentsReply, CreateAppointmentReply, CreateAppointmentRequest};
use crate::patient;
use crate::patient::patient_service_server::PatientService;
use crate::patient::{CreatePatientReply, CreatePatientRequest, PatientsReply};

pub struct MyPatientService {
    data: Arc<PostgresPatientRepo>,
}

pub struct MyAppointmentService {
    data: Arc<PostgresAppointmentRepo>,
}

impl MyPatientService {
    pub fn new(data: Arc<PostgresPatientRepo>) -> MyPatientService {
        MyPatientService { data }
    }
}

impl MyAppointmentService {
    pub fn new(data: Arc<PostgresAppointmentRepo>) -> MyAppointmentService {
        MyAppointmentService { data }
    }
}

#[tonic::async_trait]
impl PatientService for MyPatientService {
    async fn list_patients(&self, request: Request<()>) -> Result<Response<PatientsReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let patient_list = self.data.list_patients().await.unwrap_or_default();

        let reply = PatientsReply {
            patients: patient_list
                .into_iter()
                .map(|patient| patient::Patient {
                    id: patient.id,
                    first_name: patient.first_name,
                    last_name: patient.last_name,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }

    async fn create_patient(
        &self,
        request: Request<CreatePatientRequest>,
    ) -> Result<Response<CreatePatientReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let body = &request.into_inner();
        let result = self
            .data
            .add_patient(body.first_name.clone(), body.last_name.clone())
            .await;

        let reply = match result {
            Ok(0) => CreatePatientReply {
                message: format!("Failed to create patient"),
            },
            Ok(1) => CreatePatientReply {
                message: format!("Patient created!"),
            },
            _ => return Err(Status::internal("Some error occurred!")),
        };

        Ok(Response::new(reply))
    }
}

#[tonic::async_trait]
impl AppointmentService for MyAppointmentService {
    async fn list_appointments(
        &self,
        request: Request<()>,
    ) -> Result<Response<AppointmentsReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let appointment_list = self.data.list_appointments().await.unwrap_or_default();

        let reply = AppointmentsReply {
            appointments: appointment_list
                .into_iter()
                .map(|appointment| appointment::Appointment {
                    id: appointment.id,
                    start_time: Some(appointment.start_time.convert()),
                    last_name: appointment.last_name,
                })
                .collect(),
        };

        Ok(Response::new(reply))
    }

    async fn create_appointment(
        &self,
        request: Request<CreateAppointmentRequest>,
    ) -> Result<Response<CreateAppointmentReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let body = &request.into_inner();
        let result = self.data.add_appointment(body.patient_id).await;

        let reply = match result {
            Ok(0) => CreateAppointmentReply {
                message: format!("Failed to create appointment"),
            },
            Ok(1) => CreateAppointmentReply {
                message: format!("Appointment created!"),
            },
            _ => return Err(Status::internal("Some error occurred!")),
        };

        Ok(Response::new(reply))
    }
}

trait ProtoChronoExt {
    fn convert(self) -> Timestamp;
}

impl ProtoChronoExt for DateTime<Utc> {
    fn convert(self) -> Timestamp {
        let t: SystemTime = self.into();
        prost_types::Timestamp::from(t)
    }
}

// pub async fn index(
//     data: web::Data<Arc<PostgresAppointmentRepo>>,
// ) -> actix_web::Result<HttpResponse> {
//     let appointments = data.list_appointments().await.unwrap_or_default();
//     let template = Index { appointments };
//
//     Ok(HttpResponse::Ok()
//         .content_type("text/html")
//         .body(template.render().unwrap()))
// }
