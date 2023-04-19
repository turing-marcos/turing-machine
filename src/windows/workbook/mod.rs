mod book;
mod exercise;
mod wb_editor;

use eframe::egui;
const MAX_IMG_SIZE: egui::Vec2 = egui::Vec2::new(600.0, 250.0);

pub use book::BookWindow as WorkbookWindow;
pub use wb_editor::WorkbookEditorWindow;
