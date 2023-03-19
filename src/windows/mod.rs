use eframe::egui;

mod about_window;
mod debug_window;
mod error_window;
mod infinite_loop_window;
mod book;

pub use about_window::AboutWindow;
pub use debug_window::DebugWindow;
pub use error_window::ErrorWindow;
pub use infinite_loop_window::InfiniteLoopWindow;
pub use book::BookWindow;

pub trait SecondaryWindow {
    fn set_lang(&mut self, lang: &str);
    fn show(&self, ctx: &egui::Context) -> bool;
}
