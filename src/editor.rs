use std::any::Any;

use crate::app::Buffer;
use crate::cursor::Cursor;
use ropey::Rope;

#[derive(Default)]
pub struct Editor {
    pub content: Rope,
    pub cursor: Cursor,
}

impl Buffer for Editor {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let text = self.content.to_string();
            let font_id = egui::FontId::monospace(16.0);
            let text_color = ui.visuals().text_color();
            let row_height = ui.fonts_mut(|fonts| fonts.row_height(&font_id));
            let galley = ui.painter().layout_no_wrap(text, font_id, text_color);
            let size = egui::vec2(
                ui.available_width().max(galley.size().x),
                galley.size().y.max(row_height),
            );
            let (rect, _) = ui.allocate_exact_size(size, egui::Sense::click());

            ui.painter().galley(rect.min, galley.clone(), text_color);

            let cursor_rect = if galley.is_empty() {
                egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(0.0, row_height))
            } else {
                galley.pos_from_cursor(egui::text::CCursor::new(
                    self.cursor.char_index(&self.content),
                ))
            };
            let cursor_x = rect.min.x + cursor_rect.min.x;
            let cursor_y = rect.min.y + cursor_rect.min.y;

            ui.painter().line_segment(
                [
                    egui::pos2(cursor_x, cursor_y),
                    egui::pos2(cursor_x, cursor_y + cursor_rect.height().max(row_height)),
                ],
                egui::Stroke::new(1.0, text_color),
            );
        });
    }

    fn handle_input(&mut self, ui: &egui::Ui) {
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

    fn cursor_row(&self) -> usize {
        self.cursor.row
    }

    fn cursor_column(&self) -> usize {
        self.cursor.column
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
