use eframe::egui;

mod about_window;
mod debug_window;
mod error_window;

pub use about_window::AboutWindow;
pub use debug_window::DebugWindow;
pub use error_window::ErrorWindow;

pub trait SecondaryWindow {
    fn set_lang(&mut self, lang: &str);
    fn show(&self, ctx: &egui::Context) -> bool;
}
