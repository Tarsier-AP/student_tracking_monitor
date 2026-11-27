use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::models::{Computer, Note, RoomLayout, Student, StudentId, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    pub users: Vec<User>,
    pub students: Vec<Student>,
    pub computers: Vec<Computer>,
    pub notes: Vec<Note>,
    pub queue: VecDeque<StudentId>,
    pub room_layout: RoomLayout,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            students: Vec::new(),
            computers: Vec::new(),
            notes: Vec::new(),
            queue: VecDeque::new(),
            room_layout: RoomLayout {
                positions: Vec::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        ComputerAvailability, ComputerId, ComputerPosition, MessageId, Note, PresenceStatus, Role,
        StudentId, Task, TaskStatus, UserId,
    };
    use chrono::{NaiveDate, TimeZone, Utc};
    use url::Url;

    #[test]
    fn new_app_data_is_empty() {
        let data = AppData::new();

        assert!(data.users.is_empty());
        assert!(data.students.is_empty());
        assert!(data.computers.is_empty());
        assert!(data.notes.is_empty());
        assert!(data.queue.is_empty());
        assert!(data.room_layout.positions.is_empty());
    }

    #[test]
    fn app_data_can_be_serialized_and_deserialized() {
        let data = AppData::new();

        let json = serde_json::to_string(&data).unwrap();

        let loaded: AppData = serde_json::from_str(&json).unwrap();

        assert!(loaded.users.is_empty());
        assert!(loaded.students.is_empty());
        assert!(loaded.computers.is_empty());
        assert!(loaded.notes.is_empty());
        assert!(loaded.queue.is_empty());
        assert!(loaded.room_layout.positions.is_empty());
    }

    #[test]
    fn app_data_preserves_queue_order() {
        let mut data = AppData::new();

        data.queue.push_back(StudentId(10));
        data.queue.push_back(StudentId(20));
        data.queue.push_back(StudentId(30));

        let json = serde_json::to_string(&data).unwrap();

        let loaded: AppData = serde_json::from_str(&json).unwrap();

        assert_eq!(
            loaded.queue,
            VecDeque::from([StudentId(10), StudentId(20), StudentId(30),])
        );
    }

    #[test]
    fn app_data_preserves_all_data_after_roundtrip() {
        let mut data = AppData::new();

        let mut user = User::new(UserId::new(1), "Иван Иванов".into(), Role::Mentor).unwrap();

        user.set_email("ivan@example.com".into());

        let mut student = Student::new(
            StudentId::new(10),
            UserId::new(1),
            "ИС-21".into(),
            vec![Url::parse("https://github.com/example/project").unwrap()],
        )
        .unwrap();

        student.set_presence_status(PresenceStatus::AtComputer);
        student.set_task(Task {
            title: "Реализовать авторизацию".into(),
            redmine_url: Some(Url::parse("https://redmine.example.com/issues/123").unwrap()),
            status: TaskStatus::InProgress,
            deadline: Some(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()),
        });

        let mut computer = Computer::new(
            ComputerId::new(5),
            "PC-05".into(),
            "Аудитория 101".into(),
            2,
        )
        .unwrap();

        computer.set_availability(ComputerAvailability::Maintenance);
        computer.current_students.push(StudentId::new(10));

        let timestamp = Utc.with_ymd_and_hms(2026, 11, 24, 10, 0, 0).unwrap();

        let note = Note::new(
            MessageId::new(1),
            UserId::new(1),
            UserId::new(1),
            "Проверить задание".into(),
            timestamp,
        )
        .unwrap();

        data.users.push(user);
        data.students.push(student);
        data.computers.push(computer);
        data.notes.push(note);
        data.queue.push_back(StudentId::new(10));

        data.room_layout
            .add_position(ComputerPosition::new(ComputerId::new(5), 3, 4))
            .unwrap();

        let original = serde_json::to_value(&data).unwrap();
        let json = serde_json::to_string(&data).unwrap();
        let loaded: AppData = serde_json::from_str(&json).unwrap();

        assert_eq!(serde_json::to_value(loaded).unwrap(), original);
    }
}
