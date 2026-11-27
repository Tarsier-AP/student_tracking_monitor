use std::collections::VecDeque;

use crate::models::StudentId;

pub fn enqueue(queue: &mut VecDeque<StudentId>, student_id: StudentId) -> Result<(), String> {
    if queue.contains(&student_id) {
        return Err(String::from("Студент уже в очереди"));
    }

    queue.push_back(student_id);

    Ok(())
}

pub fn pop_next(queue: &mut VecDeque<StudentId>) -> Option<StudentId> {
    queue.pop_front()
}

pub fn remove(queue: &mut VecDeque<StudentId>, student_id: StudentId) -> bool {
    if let Some(index) = queue.iter().position(|id| *id == student_id) {
        queue.remove(index);
        true
    } else {
        false
    }
}

pub fn move_to_back(queue: &mut VecDeque<StudentId>, student_id: StudentId) -> bool {
    if let Some(index) = queue.iter().position(|id| *id == student_id) {
        if let Some(student) = queue.remove(index) {
            queue.push_back(student);
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_queue() -> VecDeque<StudentId> {
        VecDeque::from([StudentId(1), StudentId(2), StudentId(3)])
    }

    #[test]
    fn enqueue_adds_student_to_back() {
        let mut queue = VecDeque::new();

        assert!(enqueue(&mut queue, StudentId(1)).is_ok());
        assert_eq!(queue, VecDeque::from([StudentId(1)]));
    }

    #[test]
    fn enqueue_reject_duplicate() {
        let mut queue = make_queue();

        assert!(enqueue(&mut queue, StudentId(1)).is_err());
        assert_eq!(queue, make_queue());
    }

    #[test]
    fn pop_next_remove_first_student() {
        let mut queue = make_queue();

        assert_eq!(pop_next(&mut queue), Some(StudentId(1)));
        assert_eq!(queue, VecDeque::from([StudentId(2), StudentId(3)]));
    }

    #[test]
    fn pop_next_returns_none_for_empty_queue() {
        let mut queue = VecDeque::new();

        assert_eq!(pop_next(&mut queue), None);
    }

    #[test]
    fn remove_from_queue_existing_student() {
        let mut queue = make_queue();

        assert!(remove(&mut queue, StudentId(2)));

        assert_eq!(queue, VecDeque::from([StudentId(1), StudentId(3)]));
    }

    #[test]
    fn remove_returns_false_for_missing_student() {
        let mut queue = make_queue();

        assert!(!remove(&mut queue, StudentId(99)));
        assert_eq!(queue, make_queue());
    }

    #[test]
    fn move_to_back_returns_false_for_missing_student() {
        let mut queue = make_queue();

        assert!(!move_to_back(&mut queue, StudentId(99)));
        assert_eq!(queue, make_queue());
    }

    #[test]
    fn move_to_back_moves_student() {
        let mut queue = make_queue();

        assert!(move_to_back(&mut queue, StudentId(1)));
        assert_eq!(
            queue,
            VecDeque::from([StudentId(2), StudentId(3), StudentId(1)])
        );
    }
}
