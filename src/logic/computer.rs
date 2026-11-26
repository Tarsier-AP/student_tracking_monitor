use std::collections::VecDeque;

use crate::models::{
    Computer,
    ComputerAvailability,
    PresenceStatus,
    Student,
    StudentId
};


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