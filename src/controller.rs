use serde::{Deserialize, Serialize};
use serde_json::{Result, json};
use crate::model::*;
use crate::utils::*;

fn marshal_add_doctor_body(body: String) -> u64 {
    let mut doctor: Doctor = serde_json::from_str(body.as_str()).unwrap();
    let id = hash_str(body);
    
    doctor.id = id;
    
    add_doctor(doctor);
    
    return id
}

fn marshal_add_appointment_body(body: String) -> u64 {
    let mut appointment: Appointment = serde_json::from_str(body.as_str()).unwrap();
    let id = hash_str(body);

    appointment.id = id;
    
    add_appointment(appointment);
    
    return id
}




fn query_stringify_doctor(doctor_id: String) -> String {
    let doctor_id_u64 = doctor_id.parse::<u64>().unwrap();

    let doctor = query_doctor(doctor_id_u64);

    let doctor_json = json!(doctor);

    return doctor_json.to_string();


}

fn query_stringify_appointment(appointment_id: String) -> String {
    let appointment_id_id_u64 = appointment_id.parse::<u64>().unwrap();

    let appointment = query_appointment(appointment_id_id_u64);

    let appointment_json = json!(appointment);

    return appointment_json.to_string();


}


fn query_delete_appointment(appointment_id: String) -> String {
    let appointment_id_id_u64 = appointment_id.parse::<u64>().unwrap();

    let result = delete_appointment(appointment_id_id_u64);

    match result {
        100 => String::from("Successfully deleted!"),
        101 => String::from("There was an error"),
        _ => String::new()
    }
}



fn insert_doctor_appointment(doctor_id_str: String, appointment_id_str: String, day: String) {
    let hash_id = hash_str(format!("{}{}{}", doctor_id_str, appointment_id_str, day));

    let appointment_id_id_u32 = appointment_id_str.parse::<u32>().unwrap();
    let doctor_id_u32 = doctor_id_str.parse::<u32>().unwrap();


    let new_appointment = DoctorAppointment{id: hash_id, appointment_id: appointment_id_id_u32, doctor_id: doctor_id_u32, day_date: day};

    
    add_doctor_appointment(new_appointment)


}



