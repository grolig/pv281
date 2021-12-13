use crate::models::Appointment;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index {
    pub(crate) appointments: Vec<Appointment>,
}

mod filters {
    use chrono::{DateTime, Utc};

    // Custom filter to display datetime in html templates
    pub fn date_filter(date: &DateTime<Utc>) -> ::askama::Result<String> {
        Ok(date.format("%H:%M %d-%m-%Y").to_string())
    }
}
