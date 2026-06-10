mod app;
mod cursor;
mod editor;

fn main() {
    eframe::run_native(
        "Iglu",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(app::Iglu::new(cc)))),
    )
    .unwrap();
}
