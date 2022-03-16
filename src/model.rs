use serde::{Serialize, Deserialize};
use tinydb::Database;
use std::path::PathBuf;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
enum Specialty {
    Pediatrician,
    Endocrinologist,
    ThroatEarNose,
    Optimologist,
    Orthopedist,
    Psyhchiatrist
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
struct Doctor {
    id: u32,
    name: String,
    specialty: Specialty,
    upin: String,
    hourly_rate: u64
}
 
#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
enum Triage {
    Urgent,
    SimpleVisit,
    CheckUp,
    FirstVisit,
    PostOp
}


#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
struct Appointment {
    id: u32,
    doctor_id: u32,
    triage: Triage,
    appointment_time_unix: i64
}
 

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
struct DoctorAppointment {
    id: u32,
    day_date: String,
    doctor_id: u32,
    appointment_id: u32
}
 

fn add_doctor(doctor: Doctor) {
    let db_path = PathBuf::from("calendar_doctors.tinydb");
    let mut db: Database<Doctor> = Database::auto_from(db_path, false).unwrap();

    db.add_item(doctor.clone()).unwrap();
}

fn query_doctor(doctor_name: String) -> Doctor {
    let db_path = PathBuf::from("calendar_doctors.tinydb");
    let db: Database<Doctor> = Database::auto_from(db_path, false).unwrap();

    let default = Doctor{id: 0, name: String::from("NONE"), upin: String::from("NONE"), specialty: Specialty::Endocrinologist, hourly_rate: 0};

    let result_doctor = db.query_item(|x: &Doctor| &x.name, doctor_name).unwrap_or(&default);


    return result_doctor.clone()
}