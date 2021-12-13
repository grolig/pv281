-- Add migration script here

CREATE TABLE patient (
    id SERIAL PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL
);

CREATE TABLE appointment (
    id SERIAL PRIMARY KEY,
    patient_id INTEGER REFERENCES patient,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL
);