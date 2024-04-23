#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Define MultimediaContent struct for multimedia communication
#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct MultiMediaContent {
    image_url: Option<String>,
    video_url: Option<String>,
    audio_url: Option<String>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Student {
    id: u64,
    name: String,
    contact_details: String,
    attendance_history: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Lecture {
    id: u64,
    student_id: u64,
    lecturer_id: u64,
    date_time: u64,
    topic: String,
    multimedia_content: Option<MultiMediaContent>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct AttendanceRecord {
    id: u64,
    student_id: u64,
    attendance_status: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default, Clone)]
struct Message {
    id: u64,
    sender_id: u64,
    receiver_id: u64,
    content: String,
    multimedia_content: Option<MultiMediaContent>,
}

impl Storable for Student {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for Student {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Lecture {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for Lecture {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for AttendanceRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for AttendanceRecord {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}


impl Storable for Message {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for Message {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Define payload structs
#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct RegisterStudentPayload {
    name: String,
    contact_details: String,
    attendance_history: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ScheduleLecturePayload {
    student_id: u64,
    lecturer_id: u64,
    date_time: u64,
    topic: String,
    multimedia_content: Option<MultiMediaContent>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct UpdateStudentPayload {
    id: u64,
    name: String,
    contact_details: String,
    attendance_history: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct UpdateLecturePayload {
    lecture_id: u64,
    student_id: u64,
    lecturer_id: u64,
    date_time: u64,
    topic: String,
    multimedia_content: Option<MultiMediaContent>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct UpdateAttendanceRecordPayload {
    record_id: u64,
    student_id: u64,
    attendance_status: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ReminderPayload {
    student_id: u64,
    content: String,
    multimedia_content: Option<MultiMediaContent>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct UpdateMessage {
    message_id: u64,
    sender_id: u64,
    receiver_id: u64,
    content: String,
    multimedia_content: Option<MultiMediaContent>,
}

// Define Error enum
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    InvalidInput { msg: String },
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STUDENT_STORAGE: RefCell<StableBTreeMap<u64, Student, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static LECTURE_STORAGE: RefCell<StableBTreeMap<u64, Lecture, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));

    static ATTENDANCE_RECORD_STORAGE: RefCell<StableBTreeMap<u64, AttendanceRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));

    static MESSAGE_STORAGE: RefCell<StableBTreeMap<u64, Message, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
}

// Functions for retrieving data
#[ic_cdk::query]
fn get_student(student_id: u64) -> Result<Student, Error> {
    _get_student(&student_id)
        .ok_or_else(|| Error::NotFound { msg: format!("Student with id={} not found", student_id) })
}

#[ic_cdk::query]
fn get_lecture(lecture_id: u64) -> Result<Lecture, Error> {
    _get_lecture(&lecture_id)
        .ok_or_else(|| Error::NotFound { msg: format!("Lecture with id={} not found", lecture_id) })
}

#[ic_cdk::query]
fn get_attendance_record(record_id: u64) -> Result<AttendanceRecord, Error> {
    _get_attendance_record(&record_id)
        .ok_or_else(|| Error::NotFound { msg: format!("Attendance record with id={} not found", record_id) })
}

fn _get_student(student_id: &u64) -> Option<Student> {
    STUDENT_STORAGE.with(|service| service.borrow().get(student_id))
}

fn _get_lecture(lecture_id: &u64) -> Option<Lecture> {
    LECTURE_STORAGE.with(|service| service.borrow().get(lecture_id))
}

fn _get_attendance_record(record_id: &u64) -> Option<AttendanceRecord> {
    ATTENDANCE_RECORD_STORAGE.with(|service| service.borrow().get(record_id))
}

// Update functions
#[ic_cdk::update]
fn register_student(payload: RegisterStudentPayload) -> Result<Student, Error> {
    // Validate input data
    if payload.name.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Name cannot be empty".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let student = Student { id, 
        name: payload.name, 
        contact_details: payload.contact_details,
        attendance_history: payload.attendance_history };

    STUDENT_STORAGE.with(|service| service.borrow_mut().insert(id, student.clone()));
    Ok(student)
}

#[ic_cdk::update]
fn schedule_lecture(payload: ScheduleLecturePayload) -> Result<Lecture, Error> {
    // Validate input data
    if payload.topic.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Topic cannot be empty".to_string(),
        });
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let lecture = Lecture {
        id,
        student_id: payload.student_id,
        lecturer_id: payload.lecturer_id,
        date_time: payload.date_time,
        topic: payload.topic,
        multimedia_content: payload.multimedia_content,
    };

    LECTURE_STORAGE.with(|service| service.borrow_mut().insert(id, lecture.clone()));
    Ok(lecture)
}

#[ic_cdk::update]
fn update_student(payload: UpdateStudentPayload) -> Result<Student, Error> {
    // Validate input data
    if payload.name.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Name cannot be empty".to_string(),
        });
    }

    let updated_student = Student { 
        id: payload.id, 
        name: payload.name, 
        contact_details: payload.contact_details,
        attendance_history: payload.attendance_history
     };

    // Update student in storage
    match STUDENT_STORAGE.with(|service| service.borrow_mut().insert(payload.id, updated_student.clone())) {
        Some(_) => Ok(updated_student),
        None => Err(Error::NotFound {
            msg: format!("Student with id={} not found", payload.id),
        }),
    }
}

#[ic_cdk::update]
fn update_lecture(payload: UpdateLecturePayload) -> Result<Lecture, Error> {
    // Validate input data
    if payload.topic.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Topic cannot be empty".to_string(),
        });
    }

    let updated_lecture = Lecture {
        id: payload.lecture_id,
        student_id: payload.student_id,
        lecturer_id: payload.lecturer_id,
        date_time: payload.date_time,
        topic: payload.topic,
        multimedia_content: payload.multimedia_content,
    };

    // Update lecture in storage
    match LECTURE_STORAGE.with(|service| service.borrow_mut().insert(payload.lecture_id, updated_lecture.clone())) {
        Some(_) => Ok(updated_lecture),
        None => Err(Error::NotFound {
            msg: format!("Lecture with id={} not found", payload.lecture_id),
        }),
    }
}

#[ic_cdk::update]
fn update_attendance_record(payload: UpdateAttendanceRecordPayload) -> Result<AttendanceRecord, Error> {
    let updated_record = AttendanceRecord {
        id: payload.record_id,
        student_id: payload.student_id,
        attendance_status: payload.attendance_status,
    };

    // Update attendance record in storage
    match ATTENDANCE_RECORD_STORAGE.with(|service| service.borrow_mut().insert(payload.record_id, updated_record.clone())) {
        Some(_) => Ok(updated_record),
        None => Err(Error::NotFound {
            msg: format!("Attendance record with id={} not found", payload.record_id),
        }),
    }
}

#[ic_cdk::update]
fn send_reminder_to_student(payload: ReminderPayload) -> Result<Message, Error> {
    // Validate input data
    if payload.content.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Reminder content cannot be empty".to_string(),
        });
    }

    // Check if the student exists
    if _get_student(&payload.student_id).is_none() {
        return Err(Error::NotFound {
            msg: format!("Student with id={} not found", payload.student_id),
        });
    }

    // Get the sender ID (could be a system ID or a lecturer ID)
    let sender_id = 0; // You can change this based on your system design

    // Construct the message
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let message = Message {
        id,
        sender_id,
        receiver_id: payload.student_id,
        content: payload.content,
        multimedia_content: payload.multimedia_content,
    };

    // Store the message
    MESSAGE_STORAGE.with(|service| service.borrow_mut().insert(id, message.clone()));

    Ok(message)
}

#[ic_cdk::update]
fn update_message(payload: UpdateMessage) -> Result<Message, Error> {
    // Validate input data
    if payload.content.is_empty() {
        return Err(Error::InvalidInput {
            msg: "Message content cannot be empty".to_string(),
        });
    }

    let updated_message = Message {
        id: payload.message_id,
        sender_id: payload.sender_id,
        receiver_id: payload.receiver_id,
        content: payload.content,
        multimedia_content: payload.multimedia_content,
    };

    // Update message in storage
    match MESSAGE_STORAGE.with(|service| service.borrow_mut().insert(payload.message_id, updated_message.clone())) {
        Some(_) => Ok(updated_message),
        None => Err(Error::NotFound {
            msg: format!("Message with id={} not found", payload.message_id),
        }),
    }
}

// Delete functions
#[ic_cdk::update]
fn delete_student(student_id: u64) -> Result<(), Error> {
    // Remove student from storage
    match STUDENT_STORAGE.with(|service| service.borrow_mut().remove(&student_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Student with id={} not found", student_id),
        }),
    }
}
#[ic_cdk::update]
fn delete_lecture(lecture_id: u64) -> Result<(), Error> {
    // Remove lecture from storage
    match LECTURE_STORAGE.with(|service| service.borrow_mut().remove(&lecture_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Lecture with id={} not found", lecture_id),
        }),
    }
}

#[ic_cdk::update]
fn delete_attendance_record(record_id: u64) -> Result<(), Error> {
    // Remove attendance record from storage
    match ATTENDANCE_RECORD_STORAGE.with(|service| service.borrow_mut().remove(&record_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Attendance record with id={} not found", record_id),
        }),
    }
}

#[ic_cdk::update]
fn delete_message(message_id: u64) -> Result<(), Error> {
    // Remove message from storage
    match MESSAGE_STORAGE.with(|service| service.borrow_mut().remove(&message_id)) {
        Some(_) => Ok(()),
        None => Err(Error::NotFound {
            msg: format!("Message with id={} not found", message_id),
        }),
    }
}

// List functions
#[ic_cdk::query]
fn list_students() -> Vec<Student> {
    STUDENT_STORAGE.with(|service| service.borrow().iter().map(|(_, v)| v.clone()).collect())
}

#[ic_cdk::query]
fn list_lectures() -> Vec<Lecture> {
    LECTURE_STORAGE.with(|service| service.borrow().iter().map(|(_, v)| v.clone()).collect())
}

#[ic_cdk::query]
fn list_attendance_records() -> Vec<AttendanceRecord> {
    ATTENDANCE_RECORD_STORAGE.with(|service| service.borrow().iter().map(|(_, v)| v.clone()).collect())
}

#[ic_cdk::query]
fn list_messages() -> Vec<Message> {
    MESSAGE_STORAGE.with(|service| service.borrow().iter().map(|(_, v)| v.clone()).collect())
}

// Export Candid interface
ic_cdk::export_candid!();
