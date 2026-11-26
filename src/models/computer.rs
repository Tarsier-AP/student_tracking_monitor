use std::collections::VecDeque;

use crate::models::{PresenceStatus, Student, student};

use super::StudentId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComputerId(pub u64);

impl ComputerId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComputerAvailability {
    Online,
    Offline,
    Maintenance,
}

#[derive(Debug, Clone)]
pub struct Computer {
    pub id: ComputerId,
    pub name: String,
    pub description: Option<String>,
    pub inventory_code: Option<String>,
    pub mac: Option<String>,
    pub ip: Option<String>,
    pub room: String,
    pub availability: ComputerAvailability,
    pub max_student: usize,
    pub current_students: Vec<StudentId>,
    pub is_archived: bool,
}

pub fn set_computer_availability(
    computer: &mut Computer,
    availability: ComputerAvailability,
    students: &mut [Student],
    queue: &mut VecDeque<StudentId>,
) -> Result<(), String> {
    if computer.availability == availability {
        return Ok(());
    }

    if availability == ComputerAvailability::Online {
        computer.availability = availability;
        return Ok(());
    }

    for (index, student_id) in computer.current_students.iter().enumerate() {
        if computer.current_students[..index].contains(student_id) {
            return Err("Студент дублируется у компьютера".into());
        }

        let student = students
            .iter()
            .find(|student| student.id == *student_id)
            .ok_or_else(|| String::from("Студент не найден"))?;

        if student.presence_status != PresenceStatus::AtComputer {
            return Err("Неверный статус студента".into());
        }

        if queue.contains(student_id) {
            return Err("Студент уже в очереди".into());
        }
    }

    for student_id in &computer.current_students {
        let student = students
            .iter_mut()
            .find(|student| student.id == *student_id)
            .unwrap();

        student.presence_status = PresenceStatus::Waiting;
        queue.push_back(*student_id);
    }

    computer.current_students.clear();
    computer.availability = availability;

    Ok(())
}

impl Computer {
    pub fn new(
        id: ComputerId,
        name: String,
        room: String,
        max_student: usize,
    ) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err(String::from("Название компьютера не может быть пустым!"));
        }

        if room.trim().is_empty() {
            return Err(String::from("Аудитория не может быть пустой!"));
        }

        if max_student == 0 {
            return Err(String::from("Вместимость должна быть больше нуля!"));
        }

        Ok(Self {
            id,
            name,
            description: None,
            inventory_code: None,
            mac: None,
            ip: None,
            room,
            availability: ComputerAvailability::Online,
            max_student,
            current_students: Vec::new(),
            is_archived: false,
        })
    }

    pub fn set_name(&mut self, name: String) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err(String::from("Название компьютера не может быть пустым!"));
        }

        self.name = name;
        Ok(())
    }

    pub fn set_room(&mut self, room: String) -> Result<(), String> {
        if room.trim().is_empty() {
            return Err(String::from("Аудитория не может быть пустой!"));
        }

        self.room = room;
        Ok(())
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    pub fn clear_description(&mut self) {
        self.description = None;
    }

    pub fn set_inventory_code(&mut self, code: String) {
        self.inventory_code = Some(code);
    }

    pub fn clear_inventory_code(&mut self) {
        self.inventory_code = None;
    }

    pub fn set_mac(&mut self, mac: String) {
        self.mac = Some(mac);
    }

    pub fn clear_mac(&mut self) {
        self.mac = None;
    }

    pub fn set_ip(&mut self, ip: String) {
        self.ip = Some(ip);
    }

    pub fn clear_ip(&mut self) {
        self.ip = None;
    }

    pub fn set_availability(&mut self, availability: ComputerAvailability) {
        self.availability = availability;
    }

    pub fn set_max_student(&mut self, max_student: usize) -> Result<(), String> {
        if max_student == 0 {
            return Err(String::from("Вместимость должна быть больше нуля!"));
        }

        self.max_student = max_student;
        Ok(())
    }

    pub fn archive(&mut self) {
        self.is_archived = true;
    }

    pub fn restore(&mut self) {
        self.is_archived = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_computer() -> Computer {
        Computer::new(
            ComputerId::new(1),
            String::from("PC-01"),
            String::from("Аудитория 101"),
            2,
        )
        .unwrap()
    }

    #[test]
    fn computer_id_new_stores_value() {
        let id = ComputerId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn computer_id_value_returns_value() {
        let id = ComputerId::new(42);
        assert_eq!(id.value(), 42);
    }

    #[test]
    fn computer_id_supports_zero() {
        let id = ComputerId::new(0);
        assert_eq!(id.value(), 0);
    }

    #[test]
    fn computer_new_creates_valid_computer() {
        let computer = create_computer();

        assert_eq!(computer.id, ComputerId::new(1));
        assert_eq!(computer.name, "PC-01");
        assert_eq!(computer.room, "Аудитория 101");
        assert_eq!(computer.max_student, 2);
    }

    #[test]
    fn computer_new_defaults_to_online() {
        let computer = create_computer();
        assert_eq!(computer.availability, ComputerAvailability::Online);
    }

    #[test]
    fn computer_new_initializes_optional_fields_as_none() {
        let computer = create_computer();

        assert_eq!(computer.description, None);
        assert_eq!(computer.inventory_code, None);
        assert_eq!(computer.mac, None);
        assert_eq!(computer.ip, None);
    }

    #[test]
    fn computer_new_starts_with_no_students() {
        let computer = create_computer();
        assert!(computer.current_students.is_empty());
    }

    #[test]
    fn computer_new_is_not_archived() {
        let computer = create_computer();
        assert!(!computer.is_archived);
    }

    #[test]
    fn computer_new_rejects_empty_name() {
        let result = Computer::new(
            ComputerId::new(1),
            String::new(),
            String::from("Аудитория 101"),
            2,
        );

        assert!(result.is_err());
    }

    #[test]
    fn computer_new_rejects_whitespace_name() {
        let result = Computer::new(
            ComputerId::new(1),
            String::from("   "),
            String::from("Аудитория 101"),
            2,
        );

        assert!(result.is_err());
    }

    #[test]
    fn computer_new_rejects_empty_room() {
        let result = Computer::new(ComputerId::new(1), String::from("PC-01"), String::new(), 2);

        assert!(result.is_err());
    }

    #[test]
    fn computer_new_rejects_whitespace_room() {
        let result = Computer::new(
            ComputerId::new(1),
            String::from("PC-01"),
            String::from("   "),
            2,
        );

        assert!(result.is_err());
    }

    #[test]
    fn computer_new_rejects_zero_capacity() {
        let result = Computer::new(
            ComputerId::new(1),
            String::from("PC-01"),
            String::from("Аудитория 101"),
            0,
        );

        assert!(result.is_err());
    }

    #[test]
    fn computer_can_change_name() {
        let mut computer = create_computer();

        computer.set_name(String::from("PC-02")).unwrap();

        assert_eq!(computer.name, "PC-02");
    }

    #[test]
    fn computer_rejects_empty_name() {
        let mut computer = create_computer();

        assert!(computer.set_name(String::new()).is_err());
        assert_eq!(computer.name, "PC-01");
    }

    #[test]
    fn computer_rejects_whitespace_name() {
        let mut computer = create_computer();

        assert!(computer.set_name(String::from("   ")).is_err());
        assert_eq!(computer.name, "PC-01");
    }

    #[test]
    fn computer_can_change_room() {
        let mut computer = create_computer();

        computer.set_room(String::from("Аудитория 202")).unwrap();

        assert_eq!(computer.room, "Аудитория 202");
    }

    #[test]
    fn computer_rejects_empty_room() {
        let mut computer = create_computer();

        assert!(computer.set_room(String::new()).is_err());
        assert_eq!(computer.room, "Аудитория 101");
    }

    #[test]
    fn computer_rejects_whitespace_room() {
        let mut computer = create_computer();

        assert!(computer.set_room(String::from("   ")).is_err());
        assert_eq!(computer.room, "Аудитория 101");
    }

    #[test]
    fn computer_can_set_description() {
        let mut computer = create_computer();

        computer.set_description(String::from("Рабочая станция"));

        assert_eq!(computer.description, Some(String::from("Рабочая станция")));
    }

    #[test]
    fn computer_can_replace_description() {
        let mut computer = create_computer();

        computer.set_description(String::from("Старое описание"));
        computer.set_description(String::from("Новое описание"));

        assert_eq!(computer.description, Some(String::from("Новое описание")));
    }

    #[test]
    fn computer_can_clear_description() {
        let mut computer = create_computer();

        computer.set_description(String::from("Описание"));
        computer.clear_description();

        assert_eq!(computer.description, None);
    }

    #[test]
    fn computer_can_set_inventory_code() {
        let mut computer = create_computer();

        computer.set_inventory_code(String::from("INV-001"));

        assert_eq!(computer.inventory_code, Some(String::from("INV-001")));
    }

    #[test]
    fn computer_can_clear_inventory_code() {
        let mut computer = create_computer();

        computer.set_inventory_code(String::from("INV-001"));
        computer.clear_inventory_code();

        assert_eq!(computer.inventory_code, None);
    }

    #[test]
    fn computer_can_set_mac() {
        let mut computer = create_computer();

        computer.set_mac(String::from("AA:BB:CC:DD:EE:FF"));

        assert_eq!(computer.mac, Some(String::from("AA:BB:CC:DD:EE:FF")));
    }

    #[test]
    fn computer_can_clear_mac() {
        let mut computer = create_computer();

        computer.set_mac(String::from("AA:BB:CC:DD:EE:FF"));
        computer.clear_mac();

        assert_eq!(computer.mac, None);
    }

    #[test]
    fn computer_can_set_ip() {
        let mut computer = create_computer();

        computer.set_ip(String::from("192.168.1.10"));

        assert_eq!(computer.ip, Some(String::from("192.168.1.10")));
    }

    #[test]
    fn computer_can_clear_ip() {
        let mut computer = create_computer();

        computer.set_ip(String::from("192.168.1.10"));
        computer.clear_ip();

        assert_eq!(computer.ip, None);
    }

    #[test]
    fn computer_can_change_availability() {
        let mut computer = create_computer();

        computer.set_availability(ComputerAvailability::Offline);
        assert_eq!(computer.availability, ComputerAvailability::Offline);

        computer.set_availability(ComputerAvailability::Maintenance);
        assert_eq!(computer.availability, ComputerAvailability::Maintenance);

        computer.set_availability(ComputerAvailability::Online);
        assert_eq!(computer.availability, ComputerAvailability::Online);
    }

    #[test]
    fn computer_can_change_capacity() {
        let mut computer = create_computer();

        computer.set_max_student(5).unwrap();

        assert_eq!(computer.max_student, 5);
    }

    #[test]
    fn computer_rejects_zero_capacity() {
        let mut computer = create_computer();

        assert!(computer.set_max_student(0).is_err());
        assert_eq!(computer.max_student, 2);
    }

    #[test]
    fn computer_can_reduce_capacity_below_current_occupancy() {
        let mut computer = create_computer();

        computer.current_students = vec![StudentId::new(1), StudentId::new(2)];

        computer.set_max_student(1).unwrap();

        assert_eq!(computer.max_student, 1);
        assert_eq!(computer.current_students.len(), 2);
    }

    #[test]
    fn computer_can_archive() {
        let mut computer = create_computer();

        computer.archive();

        assert!(computer.is_archived);
    }

    #[test]
    fn computer_can_restore() {
        let mut computer = create_computer();

        computer.archive();
        computer.restore();

        assert!(!computer.is_archived);
    }

    #[test]
    fn computer_archive_is_idempotent() {
        let mut computer = create_computer();

        computer.archive();
        computer.archive();

        assert!(computer.is_archived);
    }

    #[test]
    fn computer_restore_is_idempotent() {
        let mut computer = create_computer();

        computer.restore();

        assert!(!computer.is_archived);
    }

    #[test]
    fn computer_archive_preserves_data() {
        let mut computer = create_computer();

        computer.set_description(String::from("Описание"));
        computer.set_inventory_code(String::from("INV-001"));
        computer.set_mac(String::from("AA:BB:CC:DD:EE:FF"));
        computer.set_ip(String::from("192.168.1.10"));

        computer.archive();

        assert_eq!(computer.name, "PC-01");
        assert_eq!(computer.room, "Аудитория 101");
        assert_eq!(computer.description, Some(String::from("Описание")));
        assert_eq!(computer.inventory_code, Some(String::from("INV-001")));
        assert_eq!(computer.mac, Some(String::from("AA:BB:CC:DD:EE:FF")));
        assert_eq!(computer.ip, Some(String::from("192.168.1.10")));
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::models::{
            Computer, ComputerAvailability, ComputerId, PresenceStatus, Student, StudentId, UserId,
        };
        use std::collections::VecDeque;

        fn make_computer() -> Computer {
            Computer::new(
                ComputerId(1),
                String::from("PC-01"),
                String::from("Room 1"),
                2,
            )
            .unwrap()
        }

        fn make_student(id: u64) -> Student {
            Student::new(
                StudentId(id),
                UserId(id as u64),
                String::from("Group 1"),
                Vec::new(),
            )
            .unwrap()
        }

        #[test]
        fn set_online_computer_availability() {
            let mut computer = make_computer();
            let mut students = [];
            let mut queue = VecDeque::new();

            assert!(
                set_computer_availability(
                    &mut computer,
                    ComputerAvailability::Online,
                    &mut students,
                    &mut queue,
                )
                .is_ok()
            );

            assert_eq!(computer.availability, ComputerAvailability::Online);
        }

        #[test]
        fn taking_computer_offline_returns_students_to_queue() {
            let mut computer = make_computer();
            let mut first = make_student(1);
            let mut second = make_student(2);

            first.presence_status = PresenceStatus::AtComputer;
            second.presence_status = PresenceStatus::AtComputer;

            computer.current_students = vec![first.id, second.id];

            let mut students = [first, second];
            let mut queue = VecDeque::new();

            assert!(
                set_computer_availability(
                    &mut computer,
                    ComputerAvailability::Offline,
                    &mut students,
                    &mut queue,
                )
                .is_ok()
            );

            assert_eq!(computer.availability, ComputerAvailability::Offline);
            assert!(computer.current_students.is_empty());
            assert_eq!(students[0].presence_status, PresenceStatus::Waiting);
            assert_eq!(students[1].presence_status, PresenceStatus::Waiting);
            assert_eq!(queue, VecDeque::from([StudentId(1), StudentId(2),]));
        }

        #[test]
        fn maintenance_returns_students_to_queue() {
            let mut computer = make_computer();
            let mut student = make_student(1);
            student.presence_status = PresenceStatus::AtComputer;
            computer.current_students.push(student.id);

            let mut students = [student];
            let mut queue = VecDeque::new();

            assert!(
                set_computer_availability(
                    &mut computer,
                    ComputerAvailability::Maintenance,
                    &mut students,
                    &mut queue,
                )
                .is_ok()
            );

            assert_eq!(computer.availability, ComputerAvailability::Maintenance);
            assert!(computer.current_students.is_empty());
            assert_eq!(students[0].presence_status, PresenceStatus::Waiting);
            assert_eq!(queue.front(), Some(&StudentId(1)));
        }

        #[test]
        fn setting_same_availability_does_not_change_students() {
            let mut computer = make_computer();
            let mut student = make_student(1);
            student.presence_status = PresenceStatus::AtComputer;
            computer.current_students.push(student.id);

            let mut students = [student];
            let mut queue = VecDeque::new();

            assert!(
                set_computer_availability(
                    &mut computer,
                    ComputerAvailability::Online,
                    &mut students,
                    &mut queue,
                )
                .is_ok()
            );

            assert_eq!(computer.current_students, vec![StudentId(1)]);
            assert_eq!(students[0].presence_status, PresenceStatus::AtComputer);
            assert!(queue.is_empty());
        }

        #[test]
        fn fails_if_assigned_student_is_missing() {
            let mut computer = make_computer();
            computer.current_students.push(StudentId(99));

            let mut students = [];
            let mut queue = VecDeque::new();

            assert!(
                set_computer_availability(
                    &mut computer,
                    ComputerAvailability::Offline,
                    &mut students,
                    &mut queue,
                )
                .is_err()
            );

            assert_eq!(computer.availability, ComputerAvailability::Online);
            assert_eq!(computer.current_students, vec![StudentId(99)]);
            assert!(queue.is_empty());
        }

        #[test]
        fn fails_if_student_is_already_in_queue() {
            let mut computer = make_computer();
            let mut student = make_student(1);
            student.presence_status = PresenceStatus::AtComputer;
            computer.current_students.push(student.id);

            let mut students = [student];
            let mut queue = VecDeque::from([StudentId(1)]);

            assert!(
                set_computer_availability(
                    &mut computer,
                    ComputerAvailability::Offline,
                    &mut students,
                    &mut queue,
                )
                .is_err()
            );

            assert_eq!(computer.availability, ComputerAvailability::Online);
            assert_eq!(computer.current_students, vec![StudentId(1)]);
            assert_eq!(students[0].presence_status, PresenceStatus::AtComputer);
            assert_eq!(queue, VecDeque::from([StudentId(1)]));
        }

        #[test]
        fn fails_if_student_status_is_invalid() {
            let mut computer = make_computer();
            let student = make_student(1);
            computer.current_students.push(student.id);

            let mut students = [student];
            let mut queue = VecDeque::new();

            assert!(
                set_computer_availability(
                    &mut computer,
                    ComputerAvailability::Offline,
                    &mut students,
                    &mut queue,
                )
                .is_err()
            );

            assert_eq!(computer.availability, ComputerAvailability::Online);
            assert_eq!(computer.current_students, vec![StudentId(1)]);
            assert!(queue.is_empty());
        }
    }
}
