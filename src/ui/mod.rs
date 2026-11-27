use std::path::PathBuf;

use eframe::egui::{self, Ui};

use crate::logic::{assignment, computer, queue};
use crate::models::{Computer, ComputerAvailability, PresenceStatus, Role, Student, StudentId, User};
use crate::storage::data::AppData;
use crate::storage::json::save_json;

pub struct ClassroomManager {
    data: AppData,
    path: PathBuf,
    load_error: Option<String>,
    message: Option<String>,
    selected: Option<StudentId>,
    name: String,
    group: String,
    computer_name: String,
    computer_room: String,
    seats: usize,
}

impl ClassroomManager {
    pub fn new() -> Self {
        let path = std::env::current_dir()
            .unwrap_or_default()
            .join("classroom.json");

        let (data, load_error) = match AppData::load(&path) {
            Ok(data) => (data, None),
            Err(error) => (
                AppData::default(),
                Some(format!(
                    "Не удалось загрузить {}: {error}. Файл не будет перезаписан.",
                    path.display()
                )),
            ),
        };

        Self {
            data,
            path,
            load_error,
            message: None,
            selected: None,
            name: String::new(),
            group: String::new(),
            computer_name: String::new(),
            computer_room: String::new(),
            seats: 1,
        }
    }

    fn save(&mut self) {
        if self.load_error.is_some() {
            return;
        }

        if let Err(error) = save_json(&self.path, &self.data) {
            self.message = Some(format!("Не удалось сохранить: {error}"));
        }
    }

    fn act(&mut self, result: Result<(), String>) {
        match result {
            Ok(()) => self.save(),
            Err(error) => self.message = Some(error),
        }
    }

    fn user_name(&self, student: &Student) -> String {
        self.data
            .users
            .iter()
            .find(|user| user.id == student.user_id)
            .map(|user| user.full_name.clone())
            .unwrap_or_default()
    }

    fn add_student(&mut self) {
        let user_id = self.data.new_user_id();
        let student_id = self.data.new_student_id();

        let created = User::new(user_id, self.name.trim().to_owned(), Role::Student).and_then(
            |user| {
                Student::new(
                    student_id,
                    user.id,
                    self.group.trim().to_owned(),
                    Vec::new(),
                )
                .map(|student| (user, student))
            },
        );

        match created {
            Ok((user, student)) => {
                self.data.users.push(user);
                self.data.students.push(student);
                self.name.clear();
                self.group.clear();
                self.save();
            }
            Err(error) => self.message = Some(error),
        }
    }

    fn add_computer(&mut self) {
        let id = self.data.new_computer_id();
        let created = Computer::new(
            id,
            self.computer_name.trim().to_owned(),
            self.computer_room.trim().to_owned(),
            self.seats,
        );

        match created {
            Ok(computer) => {
                self.data.computers.push(computer);
                self.computer_name.clear();
                self.computer_room.clear();
                self.seats = 1;
                self.save();
            }
            Err(error) => self.message = Some(error),
        }
    }

    fn enqueue_selected(&mut self) {
        let Some(id) = self.selected else {
            return;
        };

        let mut result = Err(String::from("Студент не выбран"));

        if let Some(student) = self.data.students.iter_mut().find(|student| student.id == id) {
            if student.is_archived {
                result = Err(String::from("Студент в архиве"));
            } else {
                result = queue::enqueue(&mut self.data.queue, id);

                if result.is_ok() {
                    student.presence_status = PresenceStatus::Waiting;
                }
            }
        }

        self.act(result);
    }

    fn dequeue_selected(&mut self) {
        let Some(id) = self.selected else {
            return;
        };

        if !queue::remove(&mut self.data.queue, id) {
            self.message = Some(String::from("Студента нет в очереди"));
            return;
        }

        if let Some(student) = self.data.students.iter_mut().find(|student| student.id == id) {
            student.presence_status = PresenceStatus::Absent;
        }

        self.save();
    }

    fn unassign_selected(&mut self) {
        let Some(id) = self.selected else {
            return;
        };

        let mut result = Err(String::from("Студент не за компьютером"));

        if let Some(student) = self.data.students.iter_mut().find(|student| student.id == id) {
            if let Some(computer) = self
                .data
                .computers
                .iter_mut()
                .find(|computer| computer.current_students.contains(&id))
            {
                result = assignment::unassign_student(student, computer, &mut self.data.queue);
            }
        }

        self.act(result);
    }

    fn assign_selected(&mut self, index: usize) {
        let Some(id) = self.selected else {
            return;
        };

        let mut result = Err(String::from("Студент не выбран"));

        if let Some(student) = self.data.students.iter_mut().find(|student| student.id == id) {
            if let Some(computer) = self.data.computers.get_mut(index) {
                result = assignment::assign_student(student, computer, &mut self.data.queue);
            }
        }

        self.act(result);
    }

    fn toggle_computer(&mut self, index: usize) {
        let Some(computer) = self.data.computers.get(index) else {
            return;
        };

        let availability = if computer.availability == ComputerAvailability::Online {
            ComputerAvailability::Offline
        } else {
            ComputerAvailability::Online
        };

        let mut result = Err(String::from("Компьютер не найден"));

        if let Some(computer) = self.data.computers.get_mut(index) {
            result = computer::set_computer_availability(
                computer,
                availability,
                &mut self.data.students,
                &mut self.data.queue,
            );
        }

        self.act(result);
    }

    fn add_forms(&mut self, ui: &mut Ui) {
        ui.heading("Новый студент");
        ui.label("ФИО");
        ui.text_edit_singleline(&mut self.name);
        ui.label("Группа");
        ui.text_edit_singleline(&mut self.group);

        if ui.button("Добавить студента").clicked() {
            self.add_student();
        }

        ui.separator();
        ui.heading("Новый компьютер");
        ui.label("Название");
        ui.text_edit_singleline(&mut self.computer_name);
        ui.label("Аудитория");
        ui.text_edit_singleline(&mut self.computer_room);
        ui.label("Мест");
        ui.add(egui::DragValue::new(&mut self.seats).range(1..=99));

        if ui.button("Добавить компьютер").clicked() {
            self.add_computer();
        }
    }

    fn draw_students(&mut self, ui: &mut Ui) {
        let students: Vec<(StudentId, String, String, PresenceStatus, bool)> = self
            .data
            .students
            .iter()
            .map(|student| {
                (
                    student.id,
                    self.user_name(student),
                    student.group.clone(),
                    student.presence_status,
                    student.is_archived,
                )
            })
            .collect();

        for (id, name, group, status, archived) in students {
            let archive = if archived { " (архив)" } else { "" };
            let label = format!("{name} — {group} — {}{archive}", status_label(status));

            if ui
                .selectable_label(self.selected == Some(id), label.as_str())
                .clicked()
            {
                self.selected = Some(id);
            }
        }
    }

    fn draw_queue(&mut self, ui: &mut Ui) {
        let queued: Vec<(StudentId, String)> = self
            .data
            .queue
            .iter()
            .map(|id| {
                let name = self
                    .data
                    .students
                    .iter()
                    .find(|student| student.id == *id)
                    .map(|student| self.user_name(student))
                    .unwrap_or_default();

                (*id, name)
            })
            .collect();

        if queued.is_empty() {
            ui.label("Пусто");
        }

        for (id, name) in queued {
            if ui
                .selectable_label(self.selected == Some(id), name.as_str())
                .clicked()
            {
                self.selected = Some(id);
            }
        }
    }

    fn draw_computers(&mut self, ui: &mut Ui) {
        let computers: Vec<(usize, String, String, usize, usize, ComputerAvailability)> = self
            .data
            .computers
            .iter()
            .enumerate()
            .map(|(index, computer)| {
                (
                    index,
                    computer.name.clone(),
                    computer.room.clone(),
                    computer.current_students.len(),
                    computer.max_student,
                    computer.availability,
                )
            })
            .collect();

        for (index, name, room, occupied, max, availability) in computers {
            let online = availability == ComputerAvailability::Online;

            ui.horizontal(|ui| {
                ui.label(format!(
                    "{name} ({room}) {occupied}/{max}{}",
                    if online { "" } else { " — выключен" }
                ));

                let can_assign = self.selected.is_some() && online && occupied < max;

                if ui
                    .add_enabled(can_assign, egui::Button::new("Назначить"))
                    .clicked()
                {
                    self.assign_selected(index);
                }

                if ui
                    .button(if online { "Выключить" } else { "Включить" })
                    .clicked()
                {
                    self.toggle_computer(index);
                }
            });
        }
    }
}

impl eframe::App for ClassroomManager {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("status").show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("ClassroomManager");

                if let Some(error) = &self.load_error {
                    ui.colored_label(egui::Color32::RED, error);
                } else if let Some(message) = &self.message {
                    ui.colored_label(egui::Color32::RED, message);
                }
            });
        });

        egui::Panel::left("forms").show(ui, |ui| self.add_forms(ui));

        egui::CentralPanel::default().show(ui, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(self.selected.is_some(), egui::Button::new("В очередь"))
                    .clicked()
                {
                    self.enqueue_selected();
                }

                if ui
                    .add_enabled(self.selected.is_some(), egui::Button::new("Освободить"))
                    .clicked()
                {
                    self.unassign_selected();
                }

                if ui
                    .add_enabled(self.selected.is_some(), egui::Button::new("Убрать из очереди"))
                    .clicked()
                {
                    self.dequeue_selected();
                }
            });

            ui.separator();
            ui.heading("Студенты");
            self.draw_students(ui);

            ui.separator();
            ui.heading("Очередь");
            self.draw_queue(ui);

            ui.separator();
            ui.heading("Компьютеры");
            self.draw_computers(ui);
        });
    }
}

fn status_label(status: PresenceStatus) -> &'static str {
    match status {
        PresenceStatus::Absent => "нет",
        PresenceStatus::Waiting => "в очереди",
        PresenceStatus::AtComputer => "за компьютером",
    }
}
