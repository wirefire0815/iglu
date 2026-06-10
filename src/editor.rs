use crate::cursor::Cursor;
use ropey::Rope;

pub fn show(ui: &mut egui::Ui, content: &Rope, cursor: &Cursor) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let text = content.to_string();
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
                galley.pos_from_cursor(egui::text::CCursor::new(cursor.char_index(content)))
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
    });
}
