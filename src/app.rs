use std::any::Any;
use std::fs;
use std::path::PathBuf;

use crate::editor::Editor;
use eframe::App;
use rfd::FileDialog;
use ropey::Rope;

pub trait Buffer: Any + Send + Sync {
    fn ui(&mut self, ui: &mut egui::Ui);
    fn handle_input(&mut self, ui: &egui::Ui);
    fn cursor_row(&self) -> usize;
    fn cursor_column(&self) -> usize;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Default, PartialEq)]
enum TabOptions {
    #[default]
    None,
    FileTab,
}

pub struct Iglu {
    open_tab: TabOptions,
    current_buffer: Box<dyn Buffer>,
}

impl Default for Iglu {
    fn default() -> Self {
        Self {
            open_tab: TabOptions::default(),
            current_buffer: Box::new(Editor::default()),
        }
    }
}

impl Iglu {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    pub fn open_file(&mut self, path: PathBuf) {
        let file_content = fs::read_to_string(path).expect("Wasn't able to read file");
        if let Some(editor) = self.current_buffer.as_any_mut().downcast_mut::<Editor>() {
            editor.content = Rope::from_str(&file_content);
        }
    }
}

impl App for Iglu {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let _ = ui.horizontal(|ui| {
            ui.selectable_value(&mut self.open_tab, TabOptions::FileTab, "File");
        });
        ui.separator();

        self.current_buffer.handle_input(ui);

        egui::Panel::bottom("status_bar").show_inside(ui, |ui| {
            ui.label(format!(
                "row: {} col: {}",
                self.current_buffer.cursor_row() + 1,
                self.current_buffer.cursor_column() + 1
            ));
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.current_buffer.ui(ui);
        });

        // TODO: this shouldn't return a path
        let res = match self.open_tab {
            TabOptions::FileTab => FileDialog::new().set_directory("/").pick_file(),
            _ => Option::None,
        };

        match res {
            Some(path) => {
                self.open_file(path);
                self.open_tab = TabOptions::None;
            }
            None => self.open_tab = TabOptions::None,
        }
    }
}
