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
        ComputerAvailability, ComputerId, PresenceStatus, Role, StudentId, UserId,
    };

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
}
