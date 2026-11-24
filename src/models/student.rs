use chrono::{DateTime, NaiveDate, Utc};
use url::Url;

use crate::models::User;

use super::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StudentId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresenceStatus {
    Absent,
    Waiting,
    AtComputer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Done,
}

#[derive(Debug, Clone)]
pub struct Task {
    pub title: String,
    pub redmine_url: Option<Url>,
    pub status: TaskStatus,
    pub deadline: Option<NaiveDate>,
}

#[derive(Debug, Clone)]
pub struct Student {
    pub id: StudentId,
    pub user_id: UserId,
    pub group: String,
    pub presence_status: PresenceStatus,
    pub task: Option<Task>,
    pub repositories: Vec<Url>,
    pub last_visit: Option<DateTime<Utc>>,
    pub last_exit: Option<DateTime<Utc>>,
    pub is_archived: bool,
}

impl StudentId {
    pub fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

impl Student {
    pub fn new(
        id: StudentId,
        user_id: UserId,
        group: String,
        repositories: Vec<Url>,
    ) -> Result<Self, String> {
        if group.trim().is_empty() {
            return Err(String::from("Группа отсуствует! Попробуйте ещё раз!"));
        }

        Ok(Self {
            id,
            user_id,
            group,
            presence_status: PresenceStatus::Absent,
            task: None,
            repositories,
            last_visit: None,
            last_exit: None,
            is_archived: false,
        })
    }

    pub fn set_group(&mut self, group: String) -> Result<(), String> {
        if group.trim().is_empty() {
            return Err(String::from("Группа отсуствует! Попробуйте ещё раз!"));
        }
        self.group = group;

        Ok(())
    }

    pub fn set_presence_status(&mut self, presence_status: PresenceStatus) {
        self.presence_status = presence_status;
    }

    pub fn set_task(&mut self, task: Task) {
        self.task = Some(task);
    }

    pub fn clear_task(&mut self) {
        self.task = None;
    }

    pub fn add_repository(&mut self, repository: Url) -> Result<(), String> {
        if self.repositories.contains(&repository) {
            return Err(String::from("Этот репозиторий уже добавлен!"));
        }

        self.repositories.push(repository);
        Ok(())
    }

    pub fn remove_repository(&mut self, repository: &Url) -> bool {
        if let Some(index) = self.repositories.iter().position(|url| url == repository) {
            self.repositories.remove(index);
            return true;
        }

        false
    }

    pub fn set_repositories(&mut self, repositories: Vec<Url>) -> Result<(), String> {
        for (index, repository) in repositories.iter().enumerate() {
            if repositories[..index].contains(repository) {
                return Err(String::from("Список содержит повторяющиеся репозитории!"));
            }
        }

        self.repositories = repositories;
        Ok(())
    }

    pub fn clear_repositories(&mut self) {
        self.repositories.clear();
    }

    pub fn set_last_visit(&mut self, last_visit: DateTime<Utc>) {
        self.last_visit = Some(last_visit)
    }

    pub fn set_last_exit(&mut self, last_exit: DateTime<Utc>) {
        self.last_exit = Some(last_exit)
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
    use chrono::{NaiveDate, TimeZone, Utc};
    use url::Url;

    fn create_student() -> Student {
        Student::new(
            StudentId::new(1),
            UserId::new(2),
            String::from("ИС-21"),
            Vec::new(),
        )
        .unwrap()
    }

    fn github_url() -> Url {
        Url::parse("https://github.com/example/project").unwrap()
    }

    fn gitlab_url() -> Url {
        Url::parse("https://gitlab.com/example/project").unwrap()
    }

    fn codeberg_url() -> Url {
        Url::parse("https://codeberg.org/example/project").unwrap()
    }

    fn create_task() -> Task {
        Task {
            title: String::from("Реализовать авторизацию"),
            redmine_url: Some(Url::parse("https://redmine.example.com/issues/123").unwrap()),
            status: TaskStatus::NotStarted,
            deadline: Some(NaiveDate::from_ymd_opt(2026, 12, 31).unwrap()),
        }
    }

    #[test]
    fn student_new_creates_student() {
        let student = create_student();

        assert_eq!(student.id, StudentId::new(1));
        assert_eq!(student.user_id, UserId::new(2));
        assert_eq!(student.group, "ИС-21");
    }

    #[test]
    fn student_new_sets_absent_status() {
        let student = create_student();

        assert_eq!(student.presence_status, PresenceStatus::Absent);
    }

    #[test]
    fn student_new_initializes_optional_fields() {
        let student = create_student();

        assert!(student.task.is_none());
        assert!(student.last_visit.is_none());
        assert!(student.last_exit.is_none());
    }

    #[test]
    fn student_new_initializes_archive_status() {
        let student = create_student();

        assert!(!student.is_archived);
    }

    #[test]
    fn student_new_accepts_empty_repositories() {
        let student = create_student();

        assert!(student.repositories.is_empty());
    }

    #[test]
    fn student_new_accepts_initial_repositories() {
        let repositories = vec![github_url(), gitlab_url()];

        let student = Student::new(
            StudentId::new(1),
            UserId::new(2),
            String::from("ИС-21"),
            repositories.clone(),
        )
        .unwrap();

        assert_eq!(student.repositories, repositories);
    }

    #[test]
    fn student_new_rejects_empty_group() {
        let result = Student::new(StudentId::new(1), UserId::new(2), String::new(), Vec::new());

        assert!(result.is_err());
    }

    #[test]
    fn student_new_rejects_whitespace_group() {
        let result = Student::new(
            StudentId::new(1),
            UserId::new(2),
            String::from("   "),
            Vec::new(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn student_can_change_group() {
        let mut student = create_student();

        assert!(student.set_group(String::from("ИС-22")).is_ok());
        assert_eq!(student.group, "ИС-22");
    }

    #[test]
    fn student_set_group_rejects_empty_value() {
        let mut student = create_student();

        assert!(student.set_group(String::new()).is_err());
        assert_eq!(student.group, "ИС-21");
    }

    #[test]
    fn student_set_group_rejects_whitespace_value() {
        let mut student = create_student();

        assert!(student.set_group(String::from("   ")).is_err());
        assert_eq!(student.group, "ИС-21");
    }

    #[test]
    fn student_can_change_presence_status() {
        let mut student = create_student();

        student.set_presence_status(PresenceStatus::Waiting);

        assert_eq!(student.presence_status, PresenceStatus::Waiting);
    }

    #[test]
    fn student_can_change_presence_status_multiple_times() {
        let mut student = create_student();

        student.set_presence_status(PresenceStatus::Waiting);
        assert_eq!(student.presence_status, PresenceStatus::Waiting);

        student.set_presence_status(PresenceStatus::AtComputer);
        assert_eq!(student.presence_status, PresenceStatus::AtComputer);

        student.set_presence_status(PresenceStatus::Absent);
        assert_eq!(student.presence_status, PresenceStatus::Absent);
    }

    #[test]
    fn student_can_set_task() {
        let mut student = create_student();
        let task = create_task();

        student.set_task(task.clone());

        assert!(student.task.is_some());
        assert_eq!(student.task.as_ref().unwrap().title, task.title,);
    }

    #[test]
    fn student_can_replace_task() {
        let mut student = create_student();

        student.set_task(create_task());

        let new_task = Task {
            title: String::from("Исправить ошибки"),
            redmine_url: None,
            status: TaskStatus::InProgress,
            deadline: None,
        };

        student.set_task(new_task.clone());

        let current_task = student.task.as_ref().unwrap();

        assert_eq!(current_task.title, new_task.title);
        assert_eq!(current_task.status, TaskStatus::InProgress);
        assert!(current_task.redmine_url.is_none());
        assert!(current_task.deadline.is_none());
    }

    #[test]
    fn student_can_clear_task() {
        let mut student = create_student();

        student.set_task(create_task());
        student.clear_task();

        assert!(student.task.is_none());
    }

    #[test]
    fn student_can_clear_missing_task() {
        let mut student = create_student();

        student.clear_task();

        assert!(student.task.is_none());
    }

    #[test]
    fn student_can_set_last_visit() {
        let mut student = create_student();

        let visit = Utc
            .with_ymd_and_hms(2026, 11, 24, 9, 30, 0)
            .single()
            .unwrap();

        student.set_last_visit(visit);

        assert_eq!(student.last_visit, Some(visit));
    }

    #[test]
    fn student_can_update_last_visit() {
        let mut student = create_student();

        let first_visit = Utc
            .with_ymd_and_hms(2026, 11, 24, 9, 30, 0)
            .single()
            .unwrap();

        let second_visit = Utc
            .with_ymd_and_hms(2026, 11, 25, 10, 0, 0)
            .single()
            .unwrap();

        student.set_last_visit(first_visit);
        student.set_last_visit(second_visit);

        assert_eq!(student.last_visit, Some(second_visit));
    }

    #[test]
    fn student_can_set_last_exit() {
        let mut student = create_student();

        let exit = Utc
            .with_ymd_and_hms(2026, 11, 24, 15, 45, 0)
            .single()
            .unwrap();

        student.set_last_exit(exit);

        assert_eq!(student.last_exit, Some(exit));
    }

    #[test]
    fn student_can_update_last_exit() {
        let mut student = create_student();

        let first_exit = Utc
            .with_ymd_and_hms(2026, 11, 24, 15, 45, 0)
            .single()
            .unwrap();

        let second_exit = Utc
            .with_ymd_and_hms(2026, 11, 25, 16, 30, 0)
            .single()
            .unwrap();

        student.set_last_exit(first_exit);
        student.set_last_exit(second_exit);

        assert_eq!(student.last_exit, Some(second_exit));
    }

    #[test]
    fn student_can_be_archived() {
        let mut student = create_student();

        student.archive();

        assert!(student.is_archived);
    }

    #[test]
    fn student_can_be_restored() {
        let mut student = create_student();

        student.archive();
        student.restore();

        assert!(!student.is_archived);
    }

    #[test]
    fn student_can_be_archived_multiple_times() {
        let mut student = create_student();

        student.archive();
        student.archive();

        assert!(student.is_archived);
    }

    #[test]
    fn student_can_be_restored_multiple_times() {
        let mut student = create_student();

        student.restore();
        student.restore();

        assert!(!student.is_archived);
    }

    #[test]
    fn restoring_student_does_not_clear_his_data() {
        let mut student = create_student();

        let repository = github_url();

        let visit = Utc
            .with_ymd_and_hms(2026, 11, 24, 9, 30, 0)
            .single()
            .unwrap();

        student.add_repository(repository.clone()).unwrap();
        student.set_task(create_task());
        student.set_last_visit(visit);

        student.archive();
        student.restore();

        assert_eq!(student.repositories, vec![repository]);
        assert!(student.task.is_some());
        assert_eq!(student.last_visit, Some(visit));
    }

    #[test]
    fn student_can_add_repository() {
        let mut student = create_student();
        let repository = github_url();

        assert!(student.add_repository(repository.clone()).is_ok());
        assert_eq!(student.repositories, vec![repository]);
    }

    #[test]
    fn student_can_add_multiple_repositories() {
        let mut student = create_student();

        let github = github_url();
        let gitlab = gitlab_url();
        let codeberg = codeberg_url();

        student.add_repository(github.clone()).unwrap();
        student.add_repository(gitlab.clone()).unwrap();
        student.add_repository(codeberg.clone()).unwrap();

        assert_eq!(student.repositories, vec![github, gitlab, codeberg],);
    }

    #[test]
    fn student_rejects_duplicate_repository() {
        let mut student = create_student();
        let repository = github_url();

        student.add_repository(repository.clone()).unwrap();

        assert!(student.add_repository(repository.clone()).is_err());
        assert_eq!(student.repositories, vec![repository]);
    }

    #[test]
    fn student_can_remove_repository() {
        let mut student = create_student();
        let repository = github_url();

        student.add_repository(repository.clone()).unwrap();

        assert!(student.remove_repository(&repository));
        assert!(student.repositories.is_empty());
    }

    #[test]
    fn student_remove_returns_false_for_missing_repository() {
        let mut student = create_student();

        let github = github_url();
        let gitlab = gitlab_url();

        student.add_repository(github.clone()).unwrap();

        assert!(!student.remove_repository(&gitlab));
        assert_eq!(student.repositories, vec![github]);
    }

    #[test]
    fn student_can_remove_one_repository_without_affecting_others() {
        let mut student = create_student();

        let github = github_url();
        let gitlab = gitlab_url();

        student.add_repository(github.clone()).unwrap();
        student.add_repository(gitlab.clone()).unwrap();

        assert!(student.remove_repository(&github));
        assert_eq!(student.repositories, vec![gitlab]);
    }

    #[test]
    fn student_can_replace_repositories() {
        let mut student = create_student();

        student.add_repository(github_url()).unwrap();

        let new_repositories = vec![gitlab_url(), codeberg_url()];

        assert!(student.set_repositories(new_repositories.clone()).is_ok());

        assert_eq!(student.repositories, new_repositories);
    }

    #[test]
    fn student_can_replace_repositories_with_empty_list() {
        let mut student = create_student();

        student.add_repository(github_url()).unwrap();

        assert!(student.set_repositories(Vec::new()).is_ok());
        assert!(student.repositories.is_empty());
    }

    #[test]
    fn student_rejects_duplicate_repositories_in_replacement() {
        let mut student = create_student();

        let original = github_url();
        student.add_repository(original.clone()).unwrap();

        let duplicate = gitlab_url();

        let result = student.set_repositories(vec![duplicate.clone(), duplicate]);

        assert!(result.is_err());
        assert_eq!(student.repositories, vec![original]);
    }

    #[test]
    fn student_can_clear_repositories() {
        let mut student = create_student();

        student.add_repository(github_url()).unwrap();
        student.add_repository(gitlab_url()).unwrap();

        student.clear_repositories();

        assert!(student.repositories.is_empty());
    }

    #[test]
    fn student_can_clear_empty_repositories() {
        let mut student = create_student();

        student.clear_repositories();

        assert!(student.repositories.is_empty());
    }
}
