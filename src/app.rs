use crate::cursor::Cursor;
use crate::editor;
use eframe::App;
use ropey::Rope;

#[derive(Default)]
pub struct Iglu {
    content: Rope,
    cursor: Cursor,
}

impl Iglu {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn handle_input(&mut self, ui: &egui::Ui) {
        ui.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Key {
                        key: egui::Key::ArrowRight,
                        pressed: true,
                        ..
                    } => {
                        let cursor_index = self.cursor.char_index(&self.content);
                        let distance_to_linefeed = self.cursor.distance_to_linefeed(&self.content);

                        if distance_to_linefeed == 0 {
                            self.cursor.row += 1;
                        }
                        self.cursor
                            .move_to_char_index(&self.content, cursor_index + 1);
                    }

                    egui::Event::Key {
                        key: egui::Key::ArrowLeft,
                        pressed: true,
                        ..
                    } => {
                        let cursor_index = self.cursor.char_index(&self.content);

                        if self.cursor.row != 0 && self.cursor.column == 0 {
                            self.cursor.row -= 1;
                        }

                        self.cursor
                            .move_to_char_index(&self.content, cursor_index.saturating_sub(1));
                    }

                    egui::Event::Key {
                        key: egui::Key::ArrowUp,
                        pressed: true,
                        ..
                    } => {
                        if self.cursor.row != 0 {
                            self.cursor.row -= 1;
                        }
                    }

                    egui::Event::Key {
                        key: egui::Key::ArrowDown,
                        pressed: true,
                        ..
                    } => {
                        if self.cursor.row < self.content.len_lines() - 1 {
                            self.cursor.row += 1;
                        }
                    }

                    egui::Event::Text(text) => {
                        let cursor_index = self.cursor.char_index(&self.content);
                        self.content.insert(cursor_index, text);
                        self.cursor
                            .move_to_char_index(&self.content, cursor_index + text.chars().count());
                    }
                    egui::Event::Key {
                        key: egui::Key::Backspace,
                        pressed: true,
                        ..
                    } => {
                        let cursor_index = self.cursor.char_index(&self.content);
                        if cursor_index > 0 {
                            let removed = self.content.char(cursor_index - 1);
                            // change cursor position after removing a linebreak
                            if removed == '\n' {
                                self.cursor.row -= 1;
                            }
                        }
                        if cursor_index > 0 {
                            self.content.remove(cursor_index - 1..cursor_index);
                            self.cursor
                                .move_to_char_index(&self.content, cursor_index - 1);
                        }
                    }
                    egui::Event::Key {
                        key: egui::Key::Enter,
                        pressed: true,
                        ..
                    } => {
                        let cursor_index = self.cursor.char_index(&self.content);
                        self.content.insert_char(cursor_index, '\n');
                        self.cursor.row += 1;
                        self.cursor
                            .move_to_char_index(&self.content, cursor_index + 1);
                    }
                    _ => {}
                }
            }
        });
    }
}

impl App for Iglu {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.handle_input(ui);
        egui::Panel::bottom("status_bar").show_inside(ui, |ui| {
            ui.label(format!(
                "row: {} col: {}",
                self.cursor.row + 1,
                self.cursor.column + 1
            ));
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                editor::show(ui, &self.content, &self.cursor);
            });
        });
    }
}
