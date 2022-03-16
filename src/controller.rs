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

fn get_doctor_appointment(doctor_id_str: String, appointment_id_str: String, day: String) -> String {
    let hash_id = hash_str(format!("{}{}{}", doctor_id_str, appointment_id_str, day));

    let doctorapp = query_doctor_appontment(hash_id);

    let doctorapp_json = json!(doctorapp);

    return doctorapp_json.to_string();
}

pub fn main_handler(body: String, url: &str, method: &str) -> String {
    let url_path = get_url_path(url);

    return match url_path {
        String::from("doctor") => {
            if method == "POST" {
                let id = marshal_add_doctor_body(body);
                id.to_string()
            } else {
                let queries = get_queries(url);
                let sel = select_query(queries, "id");

                query_stringify_doctor(sel)
            }
        },
        String::from("appointment") => {
            if method == "POST" {
                let id = marshal_add_appointment_body(body);
                id.to_string()
            } else if method == "GET" {
                let queries = get_queries(url);
                let sel = select_query(queries, "id");

                query_stringify_appointment(sel)
            } else {
                let queries = get_queries(url);
                let sel = select_query(queries, "id");

                query_delete_appointment(sel)
            }
        },
        String::from("doctorappointment") => {
            let queries = get_queries(url);
            let sel_doctor_id = select_query(queries.clone(), "doctor_id");
            let sel_appointment_id = select_query(queries.clone(), "appointment_id");
            let sel_day = select_query(queries, "doctor_day");

            return if method == "POST" {
                insert_doctor_appointment(sel_doctor_id, sel_appointment_id, sel_day);

                String::from("Success!")
            } else {
                get_doctor_appointment(sel_doctor_id, sel_appointment_id, sel_day)
            }
        }
    }
}
