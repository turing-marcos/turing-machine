mod turing_widget;
mod window;
pub mod windows;

pub use turing_widget::TuringWidget;
pub use window::MyApp;

pub struct CompositionLibrary {
    pub name: String,
    pub initial_state: String,
    pub final_state: String,
    pub code: String,
}