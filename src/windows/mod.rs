use eframe::egui;

mod about_window;
mod error_window;
mod debug_window;

pub use about_window::AboutWindow;
pub use error_window::ErrorWindow;
pub use debug_window::DebugWindow;

pub trait SecondaryWindow {
    fn show(&self, ctx: &egui::Context) -> bool;
}
