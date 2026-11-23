use eframe::egui::Ui;
use eframe::egui::ViewportBuilder;

mod logic;
mod models;
mod storage;
mod ui;

struct ClassroomManager;

impl eframe::App for ClassroomManager {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("ClassroomManager");
            ui.label("HI");
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("ClassroomManager")
            .with_inner_size([900.0, 600.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    eframe::run_native(
        "ClassroomManager",
        options,
        Box::new(|_cc| Ok(Box::new(ClassroomManager))),
    )
}
