use std::collections::VecDeque;

use crate::logic::computer::set_computer_availability;
use crate::models::{Computer, ComputerAvailability, Student, StudentId};

pub fn archive_student(
    student: &mut Student,
    computers: &[Computer],
    queue: &VecDeque<StudentId>,
) -> Result<(), String> {
    if student.is_archived {
        return Err("Student is already archived".into());
    }

    if student.presence_status == crate::models::PresenceStatus::AtComputer
        || computers
            .iter()
            .any(|computer| computer.current_students.contains(&student.id))
    {
        return Err("Student is assigned to a computer".into());
    }

    if queue.contains(&student.id) {
        return Err("Student is in the queue".into());
    }

    student.is_archived = true;
    Ok(())
}

pub fn archive_computer(
    computer: &mut Computer,
    students: &mut [Student],
    queue: &mut VecDeque<StudentId>,
) -> Result<(), String> {
    if computer.is_archived {
        return Err("Computer is already archived".into());
    }

    set_computer_availability(computer, ComputerAvailability::Offline, students, queue)?;

    computer.is_archived = true;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ComputerId, PresenceStatus, UserId};

    fn make_student(id: u64) -> Student {
        Student::new(
            StudentId(id),
            UserId(id as u64),
            String::from("Group 1"),
            Vec::new(),
        )
        .unwrap()
    }

    fn make_computer() -> Computer {
        Computer::new(
            ComputerId(1),
            String::from("PC-01"),
            String::from("Room 1"),
            2,
        )
        .unwrap()
    }

    #[test]
    fn archive_student_successfully() {
        let mut student = make_student(1);
        let computers = [];
        let queue = VecDeque::new();

        assert!(archive_student(&mut student, &computers, &queue,).is_ok());

        assert!(student.is_archived);
    }

    #[test]
    fn archive_student_fails_if_already_archived() {
        let mut student = make_student(1);
        student.is_archived = true;

        assert!(archive_student(&mut student, &[], &VecDeque::new(),).is_err());

        assert!(student.is_archived);
    }

    #[test]
    fn archive_student_fails_if_in_queue() {
        let mut student = make_student(1);
        let queue = VecDeque::from([student.id]);

        assert!(archive_student(&mut student, &[], &queue,).is_err());

        assert!(!student.is_archived);
    }

    #[test]
    fn archive_student_fails_if_assigned_to_computer() {
        let mut student = make_student(1);
        student.presence_status = PresenceStatus::AtComputer;

        let mut computer = make_computer();
        computer.current_students.push(student.id);

        assert!(archive_student(&mut student, &[computer], &VecDeque::new(),).is_err());

        assert!(!student.is_archived);
    }

    #[test]
    fn archive_computer_successfully() {
        let mut computer = make_computer();
        let mut students = [];
        let mut queue = VecDeque::new();

        assert!(archive_computer(&mut computer, &mut students, &mut queue,).is_ok());

        assert!(computer.is_archived);
        assert_eq!(computer.availability, ComputerAvailability::Offline);
    }

    #[test]
    fn archive_computer_returns_students_to_queue() {
        let mut computer = make_computer();
        let mut student = make_student(1);

        student.presence_status = PresenceStatus::AtComputer;
        computer.current_students.push(student.id);

        let mut students = [student];
        let mut queue = VecDeque::new();

        assert!(archive_computer(&mut computer, &mut students, &mut queue,).is_ok());

        assert!(computer.is_archived);
        assert!(computer.current_students.is_empty());
        assert_eq!(students[0].presence_status, PresenceStatus::Waiting);
        assert_eq!(queue, VecDeque::from([StudentId(1)]));
    }

    #[test]
    fn archive_computer_fails_if_student_is_missing() {
        let mut computer = make_computer();
        computer.current_students.push(StudentId(99));

        let mut students = [];
        let mut queue = VecDeque::new();

        assert!(archive_computer(&mut computer, &mut students, &mut queue,).is_err());

        assert!(!computer.is_archived);
        assert_eq!(computer.availability, ComputerAvailability::Online);
        assert_eq!(computer.current_students, vec![StudentId(99)]);
    }

    #[test]
    fn archive_computer_fails_if_already_archived() {
        let mut computer = make_computer();
        computer.is_archived = true;

        assert!(archive_computer(&mut computer, &mut [], &mut VecDeque::new(),).is_err());

        assert!(computer.is_archived);
    }
}
