#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Admin,
    Supervisor,
    Mentor,
    Student,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub full_name: String,
    pub role: Role,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub is_archived: bool,
}

impl UserId {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}

impl User {
    pub fn new(id: UserId, full_name: String, role: Role) -> Result<Self, String> {
        if full_name.trim().is_empty() {
            return Err(String::from("ФИО отсуствует! Попробуйте ещё раз!"));
        }

        Ok(Self {
            id,
            full_name,
            role,
            email: None,
            phone: None,
            is_archived: false,
        })
    }

    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }

    pub fn set_phone(&mut self, phone: String) {
        self.phone = Some(phone);
    }

    pub fn clear_email(&mut self) {
        self.email = None;
    }

    pub fn clear_phone(&mut self) {
        self.phone = None;
    }

    pub fn set_role(&mut self, role: Role) {
        self.role = role;
    }

    pub fn archive(&mut self) {
        self.is_archived = true;
    }

    pub fn restore(&mut self) {
        self.is_archived = false;
    }

    pub fn set_full_name(&mut self, full_name: String) -> Result<(), String> {
        if full_name.trim().is_empty() {
            return Err(String::from("ФИО отсуствует! Попробуйте ещё раз!"));
        }

        self.full_name = full_name;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::models::Role::Mentor;

    use super::*;

    #[test]
    fn user_id_new_creates_id() {
        let id = UserId::new(67);

        assert_eq!(id.0, 67);
    }
    #[test]
    fn user_id_value_returns_inner_value() {
        let id = UserId::new(67);

        assert_eq!(id.value(), 67)
    }
    #[test]
    fn user_new_creates_active_user() {
        let user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        assert_eq!(user.id, UserId::new(67));
        assert_eq!(user.full_name, "Акакий Акакиевич Башмачкин");
        assert_eq!(user.role, Role::Student);
        assert!(!user.is_archived);
    }

    #[test]
    fn user_new_has_empty_contacts() {
        let user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();
        assert_eq!(user.email, None);
        assert_eq!(user.phone, None);
    }

    #[test]
    fn user_can_set_contacts() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();
        user.set_email(String::from("example@gmail.com"));
        user.set_phone(String::from("+79999999999"));

        assert_eq!(user.email, Some(String::from("example@gmail.com")));
        assert_eq!(user.phone, Some(String::from("+79999999999")));
    }

    #[test]
    fn user_can_clear_contacts() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();
        user.set_email(String::from("example@gmail.com"));
        user.set_phone(String::from("+79999999999"));
        user.clear_email();
        user.clear_phone();

        assert_eq!(user.email, None);
        assert_eq!(user.phone, None);
    }
    #[test]
    fn user_can_change_role() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();
        user.set_role(Mentor);

        assert_eq!(user.role, Role::Mentor);
    }

    #[test]
    fn user_can_be_archived() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();
        user.archive();

        assert!(user.is_archived);
    }

    #[test]
    fn user_can_be_restored() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();
        user.archive();
        user.restore();

        assert!(!user.is_archived);
    }

    #[test]
    fn user_new_rejects_empty_name() {
        let user = User::new(UserId::new(67), String::from(""), Role::Student);

        assert!(user.is_err());

        assert_eq!(user.unwrap_err(), "ФИО отсуствует! Попробуйте ещё раз!")
    }

    #[test]
    fn user_new_rejects_whitespace_name() {
        let user = User::new(UserId::new(67), String::from("   "), Role::Student);

        assert!(user.is_err());

        assert_eq!(user.unwrap_err(), "ФИО отсуствует! Попробуйте ещё раз!")
    }

    #[test]
    fn user_can_change_full_name() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        user.set_full_name(String::from("Иван Иванов")).unwrap();

        assert_eq!(user.full_name, "Иван Иванов");
    }

    #[test]
    fn user_rejects_empty_full_name() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        let result = user.set_full_name(String::new());

        assert!(result.is_err());
        assert_eq!(user.full_name, "Акакий Акакиевич Башмачкин");
    }

    #[test]
    fn user_rejects_whitespace_full_name() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        let result = user.set_full_name(String::from("   "));

        assert!(result.is_err());
        assert_eq!(user.full_name, "Акакий Акакиевич Башмачкин");
    }

    #[test]
    fn user_can_replace_email() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        user.set_email(String::from("old@example.com"));
        user.set_email(String::from("new@example.com"));

        assert_eq!(user.email, Some(String::from("new@example.com")));
    }

    #[test]
    fn user_can_replace_phone() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        user.set_phone(String::from("+79991111111"));
        user.set_phone(String::from("+79992222222"));

        assert_eq!(user.phone, Some(String::from("+79992222222")));
    }

    #[test]
    fn user_can_be_archived_multiple_times() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        user.archive();
        user.archive();

        assert!(user.is_archived);
    }

    #[test]
    fn user_can_be_restored_multiple_times() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        user.archive();
        user.restore();
        user.restore();

        assert!(!user.is_archived);
    }

    #[test]
    fn user_can_be_archived_after_restoring() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        user.archive();
        user.restore();
        user.archive();

        assert!(user.is_archived);
    }

    #[test]
    fn user_can_change_full_name_multiple_times() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        user.set_full_name(String::from("Иван Иванов")).unwrap();
        user.set_full_name(String::from("Пётр Петров")).unwrap();

        assert_eq!(user.full_name, "Пётр Петров");
    }

    #[test]
    fn user_can_change_role_multiple_times() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        user.set_role(Role::Mentor);
        user.set_role(Role::Supervisor);

        assert_eq!(user.role, Role::Supervisor);
    }

    #[test]
    fn user_keeps_id_after_modifications() {
        let mut user = User::new(
            UserId::new(67),
            String::from("Акакий Акакиевич Башмачкин"),
            Role::Student,
        )
        .unwrap();

        user.set_full_name(String::from("Иван Иванов")).unwrap();
        user.set_email(String::from("ivan@example.com"));
        user.set_phone(String::from("+79999999999"));
        user.set_role(Role::Mentor);
        user.archive();

        assert_eq!(user.id, UserId::new(67));
    }
}
