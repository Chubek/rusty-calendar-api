use serde::{Deserialize, Serialize};
use serde_json::{Result, json};
use crate::model::*;


fn marshal_add_doctor_body(body: String) {
    let doctor: Doctor = serde_json::from_str(body.as_str()).unwrap();

    add_doctor(doctor)
}

fn marshal_add_appointment_body(body: String) {
    let appointment: Appointment = serde_json::from_str(body.as_str()).unwrap();

    add_appointment(appointment)
}


fn query_stringify_doctor(doctor_id: String) -> String {
    let doctor_id_u32 = doctor_id.parse::<u32>().unwrap();

    let doctor = query_doctor(doctor_id_u32);

    let doctor_json = json!(doctor);

    return doctor_json.to_string();


}

fn query_stringify_appointment(appointment_id: String) -> String {
    let appointment_id_id_u32 = doctor_id.parse::<u32>().unwrap();

    let appointment = query_appointment(appointment_id_id_u32);

    let appointment_json = json!(appointment);

    return appointment_json.to_string();


}

