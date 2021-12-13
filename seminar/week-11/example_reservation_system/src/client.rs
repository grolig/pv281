use patient::patient_service_client::PatientServiceClient;

pub mod patient {
    tonic::include_proto!("patient");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = PatientServiceClient::connect("http://127.0.0.1:50051").await?;

    // Create a patient
    let request = tonic::Request::new(patient::CreatePatientRequest {
        first_name: "Luke".to_string(),
        last_name: "Skywalker".to_string(),
    });
    let response = client.create_patient(request).await?;

    println!("Response from server: {:?}", response);

    Ok(())
}
