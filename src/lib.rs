#[cfg(not(target_family = "wasm"))]
mod config;
mod turing_widget;
mod window;
pub mod windows;

pub use turing_widget::TuringWidget;
pub use window::{Language, MyApp};

#[cfg(not(target_family = "wasm"))]
pub use config::Config;

pub fn get_lang() -> Language {
    match sys_locale::get_locale() {
        Some(locale) => {
            if locale[..2] == *"es" {
                Language::Spanish
            } else {
                Language::English
            }
        }
        None => Language::English,
    }
}

pub struct CompositionLibrary {
    pub name: String,
    pub initial_state: String,
    pub final_state: String,
    pub code: String,
}

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);

    // Use `js_namespace` here to bind `console.warn(..)` instead of just
    // `warn(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn warn(s: &str);

    // Use `js_namespace` here to bind `console.err(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn err(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = err)]
    fn err_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = err)]
    fn err_many(a: &str, b: &str);
}

#[cfg(target_family = "wasm")]
#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[cfg(target_family = "wasm")]
#[macro_export]
macro_rules! console_err {
    // Note that this is using the `err` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::err(&format_args!($($t)*).to_string()))
}

#[cfg(target_family = "wasm")]
#[macro_export]
macro_rules! console_warn {
    // Note that this is using the `err` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::warn(&format_args!($($t)*).to_string()))
}
