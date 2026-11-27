use std::collections::{HashSet, VecDeque};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::models::{
    Computer, ComputerId, Note, PresenceStatus, RoomLayout, Student, StudentId, User, UserId,
};
use crate::storage::json::load_json_or_default;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AppData {
    pub users: Vec<User>,
    pub students: Vec<Student>,
    pub computers: Vec<Computer>,
    pub notes: Vec<Note>,
    pub queue: VecDeque<StudentId>,
    pub room_layout: RoomLayout,
    pub next_user_id: u64,
    pub next_student_id: u64,
    pub next_computer_id: u64,
}

impl AppData {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let data: Self = load_json_or_default(path)?;
        data.validate()?;
        Ok(data)
    }

    pub fn new_user_id(&mut self) -> UserId {
        self.next_user_id += 1;
        UserId(self.next_user_id)
    }

    pub fn new_student_id(&mut self) -> StudentId {
        self.next_student_id += 1;
        StudentId(self.next_student_id)
    }

    pub fn new_computer_id(&mut self) -> ComputerId {
        self.next_computer_id += 1;
        ComputerId(self.next_computer_id)
    }

    pub fn validate(&self) -> Result<(), String> {
        let user_ids: HashSet<UserId> = self.users.iter().map(|user| user.id).collect();

        if user_ids.len() != self.users.len() {
            return Err(String::from("Пользователи с одинаковым id"));
        }

        let student_ids: HashSet<StudentId> =
            self.students.iter().map(|student| student.id).collect();

        if student_ids.len() != self.students.len() {
            return Err(String::from("Студенты с одинаковым id"));
        }

        for student in &self.students {
            if !user_ids.contains(&student.user_id) {
                return Err(format!("У студента {} нет пользователя", student.id.0));
            }

            if student
                .task
                .as_ref()
                .is_some_and(|task| task.title.trim().is_empty())
            {
                return Err(format!("У студента {} пустое задание", student.id.0));
            }
        }

        let computer_ids: HashSet<ComputerId> =
            self.computers.iter().map(|computer| computer.id).collect();

        if computer_ids.len() != self.computers.len() {
            return Err(String::from("Компьютеры с одинаковым id"));
        }

        let mut assigned = HashSet::new();

        for computer in &self.computers {
            if computer.max_student == 0 {
                return Err(format!("У компьютера {} нулевая вместимость", computer.id.0));
            }

            for student_id in &computer.current_students {
                if !assigned.insert(*student_id) {
                    return Err(format!("Студент {} назначен дважды", student_id.0));
                }

                match self.students.iter().find(|student| student.id == *student_id) {
                    Some(student) if student.presence_status == PresenceStatus::AtComputer => {}
                    Some(_) => {
                        return Err(format!(
                            "Студент {} на компьютере, но не в статусе AtComputer",
                            student_id.0
                        ));
                    }
                    None => {
                        return Err(format!(
                            "Компьютер {} ссылается на несуществующего студента",
                            computer.id.0
                        ));
                    }
                }
            }
        }

        for student in &self.students {
            if student.presence_status == PresenceStatus::AtComputer
                && !assigned.contains(&student.id)
            {
                return Err(format!(
                    "Студент {} в статусе AtComputer, но ни на одном компьютере",
                    student.id.0
                ));
            }
        }

        let mut queued = HashSet::new();

        for student_id in &self.queue {
            if !queued.insert(*student_id) {
                return Err(format!("Студент {} стоит в очереди дважды", student_id.0));
            }

            match self.students.iter().find(|student| student.id == *student_id) {
                Some(student)
                    if student.presence_status == PresenceStatus::Waiting
                        && !student.is_archived => {}
                Some(_) => {
                    return Err(format!(
                        "Студент {} в очереди с неверным статусом",
                        student_id.0
                    ));
                }
                None => {
                    return Err(format!(
                        "В очереди несуществующий студент {}",
                        student_id.0
                    ));
                }
            }
        }

        let mut placed = HashSet::new();
        let mut cells = HashSet::new();

        for position in &self.room_layout.positions {
            if !computer_ids.contains(&position.computer_id) {
                return Err(String::from("Раскладка ссылается на неизвестный компьютер"));
            }

            if !placed.insert(position.computer_id) {
                return Err(String::from("Компьютер расставлен дважды"));
            }

            if !cells.insert((position.grid_x, position.grid_y)) {
                return Err(String::from("Два компьютера стоят в одной клетке"));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        ComputerPosition, MessageId, Note, PresenceStatus, Role, StudentId, Task, TaskStatus, UserId,
    };
    use chrono::{NaiveDate, TimeZone, Utc};
    use url::Url;

    fn sample_data() -> AppData {
        let mut data = AppData::default();

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
        student
            .set_task(Task {
                title: "Реализовать авторизацию".into(),
                redmine_url: Some(Url::parse("https://redmine.example.com/issues/123").unwrap()),
                status: TaskStatus::InProgress,
                deadline: Some(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()),
            })
            .unwrap();

        let mut computer = Computer::new(
            ComputerId::new(5),
            "PC-05".into(),
            "Аудитория 101".into(),
            2,
        )
        .unwrap();

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

        data.room_layout
            .add_position(ComputerPosition::new(ComputerId::new(5), 3, 4))
            .unwrap();

        data
    }

    #[test]
    fn new_app_data_is_empty() {
        let data = AppData::default();

        assert!(data.users.is_empty());
        assert!(data.students.is_empty());
        assert!(data.computers.is_empty());
        assert!(data.notes.is_empty());
        assert!(data.queue.is_empty());
        assert!(data.room_layout.positions.is_empty());
    }

    #[test]
    fn app_data_can_be_serialized_and_deserialized() {
        let data = AppData::default();

        let json = serde_json::to_string(&data).unwrap();

        let loaded: AppData = serde_json::from_str(&json).unwrap();

        assert!(loaded.queue.is_empty());
    }

    #[test]
    fn app_data_fills_missing_fields_on_load() {
        let loaded: AppData = serde_json::from_str("{\"users\": []}").unwrap();

        assert_eq!(loaded.next_user_id, 0);
        assert!(loaded.computers.is_empty());
    }

    #[test]
    fn app_data_preserves_queue_order() {
        let mut data = AppData::default();

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
        let data = sample_data();

        let original = serde_json::to_value(&data).unwrap();
        let json = serde_json::to_string(&data).unwrap();
        let loaded: AppData = serde_json::from_str(&json).unwrap();

        assert_eq!(serde_json::to_value(loaded).unwrap(), original);
    }

    #[test]
    fn valid_data_passes_validation() {
        assert!(sample_data().validate().is_ok());
    }

    #[test]
    fn validation_rejects_zero_capacity() {
        let mut data = sample_data();
        data.computers[0].max_student = 0;

        assert!(data.validate().is_err());
    }

    #[test]
    fn validation_rejects_student_assigned_twice() {
        let mut data = sample_data();
        let mut second = data.computers[0].clone();
        second.id = ComputerId::new(6);
        second.current_students = vec![StudentId::new(10)];
        data.computers.push(second);

        assert!(data.validate().is_err());
    }

    #[test]
    fn validation_rejects_missing_user() {
        let mut data = sample_data();
        data.users.clear();

        assert!(data.validate().is_err());
    }

    #[test]
    fn validation_rejects_overlapping_layout() {
        let mut data = sample_data();
        let mut second = data.computers[0].clone();
        second.id = ComputerId::new(6);
        second.current_students.clear();
        data.computers.push(second);
        data.room_layout.positions.push(ComputerPosition::new(
            ComputerId::new(6),
            3,
            4,
        ));

        assert!(data.validate().is_err());
    }

    #[test]
    fn ids_are_unique_and_increasing() {
        let mut data = AppData::default();

        assert_eq!(data.new_student_id(), StudentId(1));
        assert_eq!(data.new_student_id(), StudentId(2));
        assert_eq!(data.new_user_id(), UserId(1));
        assert_eq!(data.new_computer_id(), ComputerId(1));
    }
}
