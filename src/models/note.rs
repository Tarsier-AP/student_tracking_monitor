use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::UserId;

id_type!(MessageId);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Note {
    pub id: MessageId,
    pub sent_by: UserId,
    pub sent_to: UserId,
    pub message: String,
    pub created_at: DateTime<Utc>,
    pub edited_at: Option<DateTime<Utc>>,
}

impl Note {
    pub fn new(
        id: MessageId,
        sent_by: UserId,
        sent_to: UserId,
        message: String,
        created_at: DateTime<Utc>,
    ) -> Result<Self, String> {
        if message.trim().is_empty() {
            return Err(String::from("Сообщение не может быть пустым!"));
        }

        Ok(Self {
            id,
            sent_by,
            sent_to,
            message,
            created_at,
            edited_at: None,
        })
    }

    pub fn edit(&mut self, message: String, edited_at: DateTime<Utc>) -> Result<(), String> {
        if message.trim().is_empty() {
            return Err(String::from("Заметка не может быть пустой!"));
        }

        self.message = message;
        self.edited_at = Some(edited_at);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn created_at() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 11, 24, 10, 0, 0).unwrap()
    }

    fn edited_at() -> DateTime<Utc> {
        Utc.with_ymd_and_hms(2026, 11, 24, 11, 0, 0).unwrap()
    }

    fn create_note() -> Note {
        Note::new(
            MessageId::new(1),
            UserId::new(2),
            UserId::new(3),
            String::from("Не забудьте выполнить задание"),
            created_at(),
        )
        .unwrap()
    }

    #[test]
    fn message_id_new_stores_value() {
        let id = MessageId::new(42);
        assert_eq!(id.0, 42);
    }

    #[test]
    fn note_new_creates_valid_note() {
        let note = create_note();

        assert_eq!(note.id, MessageId::new(1));
        assert_eq!(note.sent_by, UserId::new(2));
        assert_eq!(note.sent_to, UserId::new(3));
        assert_eq!(note.message, "Не забудьте выполнить задание");
        assert_eq!(note.created_at, created_at());
    }

    #[test]
    fn note_new_sets_edited_at_to_none() {
        let note = create_note();
        assert_eq!(note.edited_at, None);
    }

    #[test]
    fn note_new_rejects_empty_message() {
        let result = Note::new(
            MessageId::new(1),
            UserId::new(2),
            UserId::new(3),
            String::new(),
            created_at(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn note_new_rejects_whitespace_message() {
        let result = Note::new(
            MessageId::new(1),
            UserId::new(2),
            UserId::new(3),
            String::from("   \n\t  "),
            created_at(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn note_new_accepts_message_with_surrounding_whitespace() {
        let note = Note::new(
            MessageId::new(1),
            UserId::new(2),
            UserId::new(3),
            String::from("  Привет!  "),
            created_at(),
        )
        .unwrap();

        assert_eq!(note.message, "  Привет!  ");
    }

    #[test]
    fn note_edit_changes_message() {
        let mut note = create_note();

        note.edit(String::from("Новое сообщение"), edited_at())
            .unwrap();

        assert_eq!(note.message, "Новое сообщение");
    }

    #[test]
    fn note_edit_updates_edited_at() {
        let mut note = create_note();

        note.edit(String::from("Новое сообщение"), edited_at())
            .unwrap();

        assert_eq!(note.edited_at, Some(edited_at()));
    }

    #[test]
    fn note_edit_preserves_created_at() {
        let mut note = create_note();

        note.edit(String::from("Новое сообщение"), edited_at())
            .unwrap();

        assert_eq!(note.created_at, created_at());
    }

    #[test]
    fn note_edit_preserves_ids() {
        let mut note = create_note();

        note.edit(String::from("Новое сообщение"), edited_at())
            .unwrap();

        assert_eq!(note.id, MessageId::new(1));
        assert_eq!(note.sent_by, UserId::new(2));
        assert_eq!(note.sent_to, UserId::new(3));
    }

    #[test]
    fn note_edit_rejects_empty_message() {
        let mut note = create_note();

        let result = note.edit(String::new(), edited_at());

        assert!(result.is_err());
    }

    #[test]
    fn note_edit_rejects_whitespace_message() {
        let mut note = create_note();

        let result = note.edit(String::from("   \n\t"), edited_at());

        assert!(result.is_err());
    }

    #[test]
    fn note_edit_error_preserves_original_message() {
        let mut note = create_note();
        let original = note.message.clone();

        let _ = note.edit(String::new(), edited_at());

        assert_eq!(note.message, original);
    }

    #[test]
    fn note_edit_error_preserves_edited_at() {
        let mut note = create_note();

        note.edit(String::from("Первое изменение"), edited_at())
            .unwrap();

        let original_edited_at = note.edited_at;

        let _ = note.edit(String::new(), created_at());

        assert_eq!(note.edited_at, original_edited_at);
    }

    #[test]
    fn note_can_be_edited_multiple_times() {
        let mut note = create_note();

        note.edit(String::from("Первое изменение"), edited_at())
            .unwrap();

        let second_edit = Utc.with_ymd_and_hms(2026, 11, 24, 12, 0, 0).unwrap();

        note.edit(String::from("Второе изменение"), second_edit)
            .unwrap();

        assert_eq!(note.message, "Второе изменение");
        assert_eq!(note.edited_at, Some(second_edit));
    }
}
