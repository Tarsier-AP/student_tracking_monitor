use eframe::egui::ViewportBuilder;

mod logic;
mod models;
mod storage;
mod ui;

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
        Box::new(|_cc| Ok(Box::new(ui::ClassroomManager::new()))),
    )
}
