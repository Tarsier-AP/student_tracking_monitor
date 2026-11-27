use std::collections::VecDeque;

use crate::logic::queue;
use crate::models::{Computer, ComputerAvailability, PresenceStatus, Student, StudentId};

pub fn assign_student(
    student: &mut Student,
    computer: &mut Computer,
    queue: &mut VecDeque<StudentId>,
) -> Result<(), String> {
    if student.is_archived {
        return Err("Student is archived".into());
    }

    if student.presence_status == PresenceStatus::AtComputer
        || computer.current_students.contains(&student.id)
    {
        return Err("Student is already assigned".into());
    }

    if computer.is_archived {
        return Err("Computer is archived".into());
    }

    if computer.availability != ComputerAvailability::Online {
        return Err("Computer is not online".into());
    }

    if computer.current_students.len() >= computer.max_student {
        return Err("Computer is full".into());
    }

    computer.current_students.push(student.id);
    student.presence_status = PresenceStatus::AtComputer;
    queue::remove(queue, student.id);

    Ok(())
}

pub fn unassign_student(
    student: &mut Student,
    computer: &mut Computer,
    queue: &mut VecDeque<StudentId>,
) -> Result<(), String> {
    let Some(index) = computer
        .current_students
        .iter()
        .position(|id| *id == student.id)
    else {
        return Err("Student is not assigned to this computer".into());
    };

    if student.presence_status != PresenceStatus::AtComputer {
        return Err("Student status is invalid".into());
    }

    computer.current_students.remove(index);

    if let Err(error) = queue::enqueue(queue, student.id) {
        computer.current_students.insert(index, student.id);
        return Err(error);
    }

    student.presence_status = PresenceStatus::Waiting;

    Ok(())
}

pub fn move_student(
    student: &Student,
    from: &mut Computer,
    to: &mut Computer,
) -> Result<(), String> {
    if from.id == to.id {
        return Err("Source and target computers are identical".into());
    }

    if student.presence_status != PresenceStatus::AtComputer {
        return Err("Student is not assigned".into());
    }

    let Some(from_index) = from
        .current_students
        .iter()
        .position(|id| *id == student.id)
    else {
        return Err("Student is not on source computer".into());
    };

    if to.is_archived {
        return Err("Target computer is archived".into());
    }

    if to.availability != ComputerAvailability::Online {
        return Err("Target computer is not online".into());
    }

    if to.current_students.contains(&student.id) {
        return Err("Student is already on target computer".into());
    }

    if to.current_students.len() >= to.max_student {
        return Err("Target computer is full".into());
    }

    from.current_students.remove(from_index);
    to.current_students.push(student.id);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ComputerId, UserId};

    fn make_student() -> Student {
        Student::new(StudentId(1), UserId(1), String::from("Group 1"), Vec::new()).unwrap()
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

    fn make_second_computer() -> Computer {
        Computer::new(
            ComputerId(2),
            String::from("PC-02"),
            String::from("Room 1"),
            2,
        )
        .unwrap()
    }

    #[test]
    fn assign_student_successfully() {
        let mut student = make_student();
        let mut computer = make_computer();
        let mut queue = VecDeque::from([StudentId(1)]);

        assert!(assign_student(&mut student, &mut computer, &mut queue).is_ok());

        assert_eq!(computer.current_students, vec![student.id]);
        assert_eq!(student.presence_status, PresenceStatus::AtComputer);
        assert!(queue.is_empty());
    }

    #[test]
    fn assign_archived_student_fails() {
        let mut student = make_student();
        student.is_archived = true;

        let mut computer = make_computer();
        let mut queue = VecDeque::new();

        assert!(assign_student(&mut student, &mut computer, &mut queue).is_err());
        assert!(computer.current_students.is_empty());
    }

    #[test]
    fn assign_archived_computer_fails() {
        let mut student = make_student();
        let mut computer = make_computer();
        computer.is_archived = true;

        let mut queue = VecDeque::new();

        assert!(assign_student(&mut student, &mut computer, &mut queue).is_err());
        assert!(computer.current_students.is_empty());
    }

    #[test]
    fn assign_to_offline_computer_fails() {
        let mut student = make_student();
        let mut computer = make_computer();
        computer.availability = ComputerAvailability::Offline;

        let mut queue = VecDeque::new();

        assert!(assign_student(&mut student, &mut computer, &mut queue).is_err());
        assert!(computer.current_students.is_empty());
    }

    #[test]
    fn assign_to_full_computer_fails() {
        let mut student = make_student();
        let mut computer = make_computer();

        computer.current_students = vec![StudentId(2), StudentId(3)];

        let mut queue = VecDeque::new();

        assert!(assign_student(&mut student, &mut computer, &mut queue).is_err());
        assert_eq!(computer.current_students.len(), 2);
    }

    #[test]
    fn assign_already_assigned_student_fails() {
        let mut student = make_student();
        student.presence_status = PresenceStatus::AtComputer;

        let mut computer = make_computer();
        let mut queue = VecDeque::new();

        assert!(assign_student(&mut student, &mut computer, &mut queue).is_err());
        assert!(computer.current_students.is_empty());
    }

    #[test]
    fn unassign_student_returns_student_to_queue() {
        let mut student = make_student();
        student.presence_status = PresenceStatus::AtComputer;

        let mut computer = make_computer();
        computer.current_students.push(student.id);

        let mut queue = VecDeque::new();

        assert!(unassign_student(&mut student, &mut computer, &mut queue).is_ok());

        assert!(computer.current_students.is_empty());
        assert_eq!(student.presence_status, PresenceStatus::Waiting);
        assert_eq!(queue, VecDeque::from([student.id]));
    }

    #[test]
    fn unassign_student_fails_if_not_assigned() {
        let mut student = make_student();
        let mut computer = make_computer();
        let mut queue = VecDeque::new();

        assert!(unassign_student(&mut student, &mut computer, &mut queue).is_err());

        assert!(computer.current_students.is_empty());
        assert!(queue.is_empty());
    }

    #[test]
    fn unassign_student_fails_if_status_is_invalid() {
        let mut student = make_student();

        let mut computer = make_computer();
        computer.current_students.push(student.id);

        let mut queue = VecDeque::new();

        assert!(unassign_student(&mut student, &mut computer, &mut queue).is_err());

        assert_eq!(computer.current_students, vec![student.id]);
        assert!(queue.is_empty());
    }

    #[test]
    fn unassign_student_rolls_back_if_already_in_queue() {
        let mut student = make_student();
        student.presence_status = PresenceStatus::AtComputer;

        let mut computer = make_computer();
        computer.current_students.push(student.id);

        let mut queue = VecDeque::from([student.id]);

        assert!(unassign_student(&mut student, &mut computer, &mut queue).is_err());

        assert_eq!(computer.current_students, vec![student.id]);
        assert_eq!(student.presence_status, PresenceStatus::AtComputer);
        assert_eq!(queue, VecDeque::from([student.id]));
    }

    #[test]
    fn move_student_between_computers() {
        let mut student = make_student();
        student.presence_status = PresenceStatus::AtComputer;

        let mut from = make_computer();
        from.current_students.push(student.id);

        let mut to = make_second_computer();

        assert!(move_student(&student, &mut from, &mut to).is_ok());

        assert!(from.current_students.is_empty());
        assert_eq!(to.current_students, vec![student.id]);
        assert_eq!(student.presence_status, PresenceStatus::AtComputer);
    }

    #[test]
    fn move_student_fails_when_target_is_full() {
        let mut student = make_student();
        student.presence_status = PresenceStatus::AtComputer;

        let mut from = make_computer();
        from.current_students.push(student.id);

        let mut to = make_second_computer();
        to.current_students = vec![StudentId(99), StudentId(100)];

        assert!(move_student(&student, &mut from, &mut to).is_err());

        assert_eq!(from.current_students, vec![student.id]);
        assert_eq!(to.current_students, vec![StudentId(99), StudentId(100)]);
    }

    #[test]
    fn move_student_fails_when_source_is_missing_student() {
        let mut student = make_student();
        student.presence_status = PresenceStatus::AtComputer;

        let mut from = make_computer();
        let mut to = make_second_computer();

        assert!(move_student(&student, &mut from, &mut to).is_err());

        assert!(from.current_students.is_empty());
        assert!(to.current_students.is_empty());
    }

    #[test]
    fn move_student_fails_when_target_is_offline() {
        let mut student = make_student();
        student.presence_status = PresenceStatus::AtComputer;

        let mut from = make_computer();
        from.current_students.push(student.id);

        let mut to = make_second_computer();
        to.availability = ComputerAvailability::Offline;

        assert!(move_student(&student, &mut from, &mut to).is_err());

        assert_eq!(from.current_students, vec![student.id]);
        assert!(to.current_students.is_empty());
    }

    #[test]
    fn move_student_fails_when_target_is_archived() {
        let mut student = make_student();
        student.presence_status = PresenceStatus::AtComputer;

        let mut from = make_computer();
        from.current_students.push(student.id);

        let mut to = make_second_computer();
        to.is_archived = true;

        assert!(move_student(&student, &mut from, &mut to).is_err());

        assert_eq!(from.current_students, vec![student.id]);
        assert!(to.current_students.is_empty());
    }

    #[test]
    fn move_student_fails_when_computers_are_identical() {
        let mut student = make_student();
        student.presence_status = PresenceStatus::AtComputer;

        let mut from = make_computer();
        from.current_students.push(student.id);

        let mut to = make_second_computer();

        to.id = from.id;

        assert!(move_student(&student, &mut from, &mut to).is_err());

        assert_eq!(from.current_students, vec![student.id]);
        assert!(to.current_students.is_empty());
    }
}

