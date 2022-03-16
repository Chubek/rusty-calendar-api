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
pub struct Doctor {
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
pub struct Appointment {
    id: u32,
    doctor_id: u32,
    triage: Triage,
    appointment_time_unix: i64
}
 

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Clone)]
pub struct DoctorAppointment {
    id: u32,
    day_date: String,
    doctor_id: u32,
    appointment_id: u32
}
 

pub fn add_doctor(doctor: Doctor) {
    let db_path = PathBuf::from("calendar_doctors.tinydb");
    let mut db: Database<Doctor> = Database::auto_from(db_path, false).unwrap();

    db.add_item(doctor.clone()).unwrap();
}

pub fn query_doctor(doctor_id: u32) -> Doctor {
    let db_path = PathBuf::from("calendar_doctors.tinydb");
    let db: Database<Doctor> = Database::auto_from(db_path, false).unwrap();

    let default = Doctor{id: 0, name: String::from("NONE"), upin: String::from("NONE"), specialty: Specialty::Endocrinologist, hourly_rate: 0};

    let result_doctor = db.query_item(|x: &Doctor| &x.id, doctor_id).unwrap_or(&default);


    return result_doctor.clone()
}

pub fn add_appointment(appointment: Appointment) {
    let db_path = PathBuf::from("calendar_appointments.tinydb");
    let mut db: Database<Appointment> = Database::auto_from(db_path, false).unwrap();

    db.add_item(appointment.clone()).unwrap();
}

pub fn query_appointment(appointment_id: u32) -> Appointment {
    let db_path = PathBuf::from("calendar_appointments.tinydb");
    let db: Database<Appointment> = Database::auto_from(db_path, false).unwrap();

    let default = Appointment{id: 0, doctor_id: 0, triage: Triage::CheckUp, appointment_time_unix: 0};

    let result_appointment = db.query_item(|x: &Appointment| &x.id, appointment_id).unwrap_or(&default);


    return result_appointment.clone()
}

pub fn add_doctor_appointment(doctor_appointment: DoctorAppointment) {
    let db_path = PathBuf::from("calendar_doctor_appointments.tinydb");
    let mut db: Database<DoctorAppointment> = Database::auto_from(db_path, false).unwrap();

    db.add_item(doctor_appointment.clone()).unwrap();
}

pub fn query_doctor_appontment(doctor_appointment_id: u32) -> DoctorAppointment {
    let db_path = PathBuf::from("calendar_doctor_appointments.tinydb");
    let db: Database<DoctorAppointment> = Database::auto_from(db_path, false).unwrap();

    let default = DoctorAppointment{id: 0, appointment_id: 0, doctor_id: 0, day_date: String::from("NONE")};

    let result_doctor_appointment = db.query_item(|x: &DoctorAppointment| &x.id, doctor_appointment_id).unwrap_or(&default);


    return result_doctor_appointment.clone()
}