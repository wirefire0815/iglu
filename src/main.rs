use eframe::App;
use ropey::Rope;

pub fn main() {
    eframe::run_native(
        "Iglu",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(Iglu::new(cc)))),
    )
    .unwrap();
}

#[derive(Default)]
struct Iglu {
    content: Rope,
}

impl Iglu {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl App for Iglu {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Text(text) => {
                        self.content.insert(self.content.len_chars(), text);
                    }
                    egui::Event::Key {
                        key: egui::Key::Backspace,
                        pressed: true,
                        ..
                    } => {
                        let len = self.content.len_chars();
                        if len > 0 {
                            self.content.remove(len - 1..len);
                        }
                    }
                    egui::Event::Key {
                        key: egui::Key::Enter,
                        pressed: true,
                        ..
                    } => {
                        self.content.insert_char(self.content.len_chars(), '\n');
                    }
                    _ => {}
                }
            }
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label(egui::RichText::new(self.content.to_string()).monospace());
            });
        });
    }
}
