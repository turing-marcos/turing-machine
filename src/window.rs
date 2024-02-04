use std::{
    fmt::Display,
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{
    console_err, console_log, console_warn,
    windows::{
        AboutWindow, CompositionHelpWindow, DebugWindow, InfiniteLoopWindow, SecondaryWindow,
        WorkbookEditorWindow, WorkbookWindow,
    },
    TuringWidget,
};

use eframe::egui::{self, Id, RichText, TextEdit, Ui};
use eframe::epaint::Color32;
use internationalization::t;
use turing_lib::TuringOutput;
use turing_lib::{CompilerError, TuringMachine};

#[cfg(not(target_family = "wasm"))]
use {
    crate::Config,
    log::{debug, error, warn},
    serde::{Deserialize, Serialize},
    std::{
        fs::{self, File},
        io::Write,
    },
};

#[cfg(target_family = "wasm")]
use {crate::get_lang, poll_promise::Promise, wasm_bindgen::prelude::wasm_bindgen};

const DEFAULT_CODE: &str = include_str!("../Examples/Example1.tm");
const MOBILE_THRESHOLD: f32 = 500.0;

pub fn is_mobile(ctx: &egui::Context) -> bool {
    ctx.screen_rect().width() < MOBILE_THRESHOLD
}

#[cfg(target_family = "wasm")]
#[wasm_bindgen(module = "/assets/utils.js")]
extern "C" {
    fn downloadToFile(content: &str, filename: &str);
}

#[cfg(not(target_family = "wasm"))]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Language {
    English,
    Spanish,
}

#[cfg(target_family = "wasm")]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Language {
    English,
    Spanish,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::English => "en",
                Language::Spanish => "es",
            }
        )
    }
}

pub struct MyApp<'a> {
    code: String,
    error: Option<CompilerError>,
    tm: TuringWidget,

    // Windows
    about_window: Option<Box<AboutWindow>>,
    debug_window: Option<Box<DebugWindow>>,
    infinite_loop_window: Option<Box<InfiniteLoopWindow>>,
    book_window: Option<Box<WorkbookWindow<'a>>>,
    workbook_editor_window: Option<Box<WorkbookEditorWindow<'a>>>,
    composition_help_window: Option<Box<CompositionHelpWindow>>,

    #[cfg(not(target_family = "wasm"))]
    config: Config,
    #[cfg(not(target_family = "wasm"))]
    autosave: bool,

    saved_feedback: Option<Instant>,

    #[cfg(not(target_family = "wasm"))]
    file: Option<PathBuf>,

    #[cfg(target_family = "wasm")]
    lang: Language,

    #[cfg(target_family = "wasm")]
    file_request_future: Option<Promise<Option<String>>>,
}

impl<'a> MyApp<'static> {
    pub fn new(
        file: &Option<PathBuf>,
        cc: &eframe::CreationContext<'_>,
    ) -> Result<Self, CompilerError> {
        let code = if cfg!(target_family = "wasm") {
            DEFAULT_CODE.to_string()
        } else {
            match file {
                Some(ref _f) => {
                    #[cfg(not(target_family = "wasm"))]
                    {
                        console_log!("File provided: {:?}", file);

                        fs::read_to_string(_f).expect("cannot read file")
                    }

                    #[cfg(target_family = "wasm")]
                    DEFAULT_CODE.to_string()
                }
                None => {
                    console_log!("No file provided, opening an example");

                    DEFAULT_CODE.to_string()
                }
            }
        };

        let (tm, warnings) = match TuringMachine::new(&code) {
            Ok((t, warnings)) => {
                for w in &warnings {
                    console_warn!("\tCompiler warning: {:?}", w)
                }

                console_log!("Turing machine created successfully");

                (t, warnings)
            }
            Err(e) => {
                return Err(e);
            }
        };

        let mut st = (*egui::Context::default().style()).clone();
        st.override_font_id = Some(egui::FontId::monospace(14.0));
        st.spacing.slider_width = 250.0;
        st.spacing.button_padding = egui::Vec2::new(10.0, 5.0);
        st.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);
        cc.egui_ctx.set_style(st);

        #[cfg(not(target_family = "wasm"))]
        {
            let config = match Config::load() {
                Some(c) => c,
                None => {
                    console_log!("The config file did not exist, creating a default one");

                    let c = Config::default();

                    c.save();

                    c
                }
            };

            Ok(Self {
                code: String::from(&tm.code),
                error: None,
                tm: TuringWidget::new(tm, warnings).set_config(&config),
                about_window: None,
                debug_window: None,
                infinite_loop_window: None,
                book_window: None,
                workbook_editor_window: None,
                composition_help_window: None,

                config,
                autosave: file.is_some() && config.autosave_disabled(),

                file: file.clone(),
                saved_feedback: None,
            })
        }

        #[cfg(target_family = "wasm")]
        {
            Ok(Self {
                code: String::from(&tm.code),
                error: None,
                tm: TuringWidget::new(tm, warnings),
                about_window: None,
                debug_window: None,
                infinite_loop_window: None,
                book_window: None,
                workbook_editor_window: None,
                composition_help_window: None,

                lang: get_lang(),

                saved_feedback: None,

                file_request_future: None,
            })
        }
    }

    pub fn get_lang(&self) -> String {
        #[cfg(not(target_family = "wasm"))]
        {
            self.config.language().to_string()
        }

        #[cfg(target_family = "wasm")]
        {
            self.lang.to_string()
        }
    }

    /// This function handles parsing errors in the Turing machine's input and displays an error message
    /// in the UI, showing information about the location and nature of the error.
    ///
    /// # Arguments
    /// * _ui - A mutable reference to the egui Ui, used to build and update the user interface.
    /// * ctx - A reference to the egui::Context, providing access to the UI context.
    /// * error - A reference to a pest::error::Error, containing information about the parsing error.
    ///
    /// The error panel is displayed at the bottom of the UI, showing the line and column numbers
    /// where the error occurred, the erroneous input, and a message describing the expected input or
    /// the reason for the error. The panel uses different text colors and sizes to improve readability
    /// and highlight the most important information.
    fn handle_error(_ui: &mut Ui, ctx: &egui::Context, error: &CompilerError) {
        egui::TopBottomPanel::bottom("error").show(ctx, |ui| {
            egui::Frame::none()
                .fill(Color32::BLACK)
                .inner_margin(egui::style::Margin::same(10.0))
                .outer_margin(egui::style::Margin::same(0.0))
                .show(ui, |ui: &mut egui::Ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(error.code().to_string())
                                    .color(Color32::WHITE)
                                    .size(20.0),
                            );
                        });

                        ui.horizontal(|ui| {
                            let position = error.position();

                            ui.label(
                                RichText::new(format!(
                                    "{:~>width1$}{:^<width2$}{:~<width3$}",
                                    "~",
                                    "^",
                                    "~",
                                    // Length from the start of the line to the error
                                    width1 = position.start.1,
                                    // Length of the error
                                    width2 = position.end.unwrap_or((0, position.start.1 + 1)).1
                                        - position.start.1,
                                    // Length from the end of the error to the end of the line
                                    width3 = error.code().len().saturating_sub(
                                        position.end.unwrap_or((0, position.start.1 + 1)).1
                                    )
                                ))
                                .color(Color32::RED)
                                .size(20.0),
                            );

                            ui.label(
                                RichText::new(error.get_message_expected())
                                    .color(Color32::DARK_RED)
                                    .size(20.0),
                            );
                        });
                    });
                });
        });
    }

    /// This function processes the Turing machine's controls, handling UI updates and animations
    /// for stepping through the machine's operations. It enables or disables the UI elements based on the
    /// editor_focused parameter, and handles the step button click, arrow key press, and machine state changes.
    /// It also manages the tape animation based on the current offset and animation speed.
    ///
    /// # Arguments
    /// * ui - A mutable reference to the egui Ui, used to build and update the user interface.
    /// * ctx - A reference to the egui::Context, providing access to the UI context.
    /// * editor_focused - A boolean flag indicating whether the editor is currently focused.
    /// * lang - A string slice representing the current language, used for text localization.
    ///
    /// # Returns
    /// A boolean value indicating whether the UI state has changed and requires a redraw.
    fn process_turing_controls(
        &mut self,
        ui: &mut Ui,
        ctx: &egui::Context,
        editor_focused: bool,
        lang: &str,
    ) -> bool {
        ui.add_enabled_ui(!editor_focused, |ui| {
            if self.tm.offset != 0.0 {
                ui.add_enabled(false, |ui: &mut Ui| ui.button(t!("lbl.machine.step", lang)))
                    .on_hover_text_at_pointer(t!("tooltip.main.step", lang));

                if self.tm.offset.abs() < 0.01 {
                    self.tm.offset = 0.0;
                    false
                } else {
                    self.tm.offset = ctx.animate_value_with_time(
                        Id::new("offset"),
                        0.0,
                        self.tm.tape_anim_speed,
                    );
                    true
                }
            } else if (ui
                .add_enabled(self.tm.paused, |ui: &mut Ui| {
                    ui.button(t!("lbl.machine.step", lang))
                        .on_hover_text_at_pointer(t!("tooltip.main.step", lang))
                })
                .clicked()
                || ui.input(|i| i.key_pressed(egui::Key::ArrowRight))
                || !self.tm.paused)
                && !editor_focused
            {
                ctx.clear_animations();

                let target = self.tm.step();

                ctx.animate_value_with_time(Id::new("offset"), target, self.tm.tape_anim_speed);
                return true;
            } else {
                return false;
            }
        })
        .inner
    }

    /// This method restarts the Turing machine with the provided code. It attempts to parse the new code
    /// and update the Turing machine's state accordingly. If the parsing is successful, the Turing machine
    /// is updated and any previous error information is cleared. If an error occurs during parsing, the error
    /// information is stored, and the Turing machine retains its previous state.
    ///
    /// # Arguments
    /// * code - A string slice containing the new code for the Turing machine.
    ///
    /// The method updates the tm field with a new Turing machine instance or keeps the old instance if an
    /// error occurs. It also updates the code field with the provided code and manages the error field
    /// based on the success or failure of parsing the new code.
    pub fn restart(&mut self, code: &str) {
        self.tm = match self.tm.restart(code) {
            Ok(t) => {
                self.error = None;
                t
            }
            Err(e) => {
                self.error = Some(e);
                self.tm.clone()
            }
        };
        self.code = String::from(code);
    }

    /// The method checks if a file is associated with the Turing machine's code. If there is an associated
    /// file, it attempts to create and write the file with the current code. If the write operation is
    /// successful, the method returns the current Instant. If an error occurs during the save operation,
    /// the method logs the error and returns None.
    ///
    /// # Returns
    /// An Option<Instant> representing the time the file was saved if the save operation is successful,
    /// or None if the save operation fails or if there is no associated file.
    #[cfg(not(target_family = "wasm"))]
    fn auto_save_file(&self) -> Option<Instant> {
        if let Some(file) = &self.file {
            if let Ok(mut file) = File::create(file) {
                if let Err(e) = file.write_all(self.code.as_bytes()) {
                    console_err!("Error saving file: {}", e);
                } else {
                    console_log!("File saved");
                    return Some(Instant::now());
                }
            } else {
                console_err!("Error opening file \"{}\" for writing", file.display());
            }
        }

        None
    }

    /// The method checks if there's a saved_feedback timestamp, indicating that a save operation has occurred.
    /// If so, it calculates the elapsed time since the save operation and determines the appropriate alpha value
    /// for the fade-in and fade-out animations. The popup is positioned above the current ui.cursor(), with its
    /// position adjusted based on the available height. The popup consists of a rectangle with rounded corners and
    /// the "Saved file" text inside. The method also schedules a repaint after a short duration to ensure smooth
    /// animation. Once the total duration of the animation has passed, the saved_feedback value is set to None.
    ///
    /// # Arguments
    /// * ui - A mutable reference to the egui Ui, used to build and update the user interface.
    /// * ctx - A reference to the egui::Context, providing access to the UI context.    
    fn draw_saved_feedback_popup(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        if let Some(start_time) = self.saved_feedback {
            let elapsed = start_time.elapsed().as_secs_f32();

            // Set the duration of the fade-in and fade-out animations.
            let fade_in_duration = 0.25;
            let fade_out_duration = 1.0;
            let visible_duration = 1.0;
            let total_duration = fade_in_duration + fade_out_duration + visible_duration;

            if elapsed < total_duration {
                let alpha = if elapsed < fade_in_duration {
                    (elapsed / fade_in_duration).clamp(0.0, 1.0)
                } else if elapsed > fade_in_duration + visible_duration {
                    ((total_duration - elapsed) / fade_in_duration).clamp(0.0, 1.0)
                } else {
                    1.0
                };

                let color = Color32::from_white_alpha((alpha * 255.0) as u8);

                // Modify cursor position to make the popup appear above and on top
                let cursor_pos = ui.cursor();
                let popup_size = egui::Vec2::new(100.0, 30.0);
                let popup_pos: egui::Pos2 = if ui.available_height() == 0.0 {
                    egui::Pos2::new(
                        cursor_pos.center().x - popup_size.x / 2.0,
                        cursor_pos.top() - 200.0,
                    )
                } else {
                    egui::Pos2::new(cursor_pos.center().x - popup_size.x / 2.0, cursor_pos.top())
                };

                let rect = egui::Rect::from_min_size(popup_pos, popup_size);
                ui.allocate_space(rect.size());
                let mut popup_ui = ui.child_ui(rect, egui::Layout::default());

                egui::Frame::popup(&ctx.style())
                    .fill(Color32::from_rgba_unmultiplied(0, 0, 0, 150))
                    .show(&mut popup_ui, |ui| {
                        ui.label(egui::RichText::new("Saved file").color(color));
                    });

                ctx.request_repaint_after(Duration::from_millis(50));
            } else {
                self.saved_feedback = None;
            }
        }
    }

    /// This method saves the current code to an associated file, or spawns a dialog to select a file and
    /// saves the code there. The method handles both WebAssembly and non-WebAssembly targets.
    ///
    /// For WebAssembly targets, an async file dialog is spawned on the main thread, and the code is saved
    /// to the selected file using the write_all method in an async context.
    ///
    /// For non-WebAssembly targets, the method checks if there is an associated file. If there is, it saves
    /// the code to that file. If there isn't, it spawns a file dialog to select a file, sets the file filter
    /// to "TuringMachine" with a ".tm" extension, and saves the code to the selected file using std::fs::write.
    ///
    /// After a successful save operation, the file is set as the new auto-save file. If the save operation
    /// fails, an error message is logged.
    #[cfg(not(target_family = "wasm"))]
    fn save_file(&mut self) {
        let file: Option<PathBuf> = match &self.file {
            Some(f) => Some(f.clone()),
            None => {
                let path = std::env::current_dir().unwrap();

                rfd::FileDialog::new()
                    .add_filter("TuringMachine", &["tm"])
                    .set_directory(path)
                    .save_file()
            }
        };

        if let Some(f) = file {
            std::fs::write(&f, self.code.as_bytes()).expect("cannot write file");
            self.file = Some(f);

            console_log!("Set auto-save file to {:?}", self.file);

            self.saved_feedback = Some(Instant::now());
        } else {
            console_err!("Cannot save file");
        }
    }

    /// This method spawns a dialog to select a file and saves the code there.
    /// The method handles both WebAssembly and non-WebAssembly targets.
    ///
    /// For WebAssembly targets, an async file dialog is spawned on the main thread, and the code is saved
    /// to the selected file using the write_all method in an async context.
    ///
    /// For non-WebAssembly targets, the method spawns a file dialog to select a file, sets the file filter
    /// to "TuringMachine" with a ".tm" extension, and saves the code to the selected file using std::fs::write.
    ///
    /// After a successful save operation, the file is set as the new auto-save file. If the save operation
    /// fails, an error message is logged.
    #[cfg(not(target_family = "wasm"))]
    fn save_file_as(&mut self) {
        {
            let path = std::env::current_dir().unwrap();

            let file = rfd::FileDialog::new()
                .add_filter("TuringMachine", &["tm"])
                .set_directory(path)
                .save_file();

            if let Some(f) = file {
                std::fs::write(&f, self.code.as_bytes()).expect("cannot write file");
                self.file = Some(f);

                console_log!("Set auto-save file to {:?}", self.file);

                self.saved_feedback = Some(Instant::now());
            } else {
                console_err!("Cannot save file");
            }
        }
    }

    /// This method loads the code from an associated file, or spawns a dialog to select a file and then
    /// loads the code from it. The method handles both WebAssembly and non-WebAssembly targets.
    ///
    /// For WebAssembly targets, an async file dialog is spawned on the main thread, and the code is read
    /// from the selected file using the read method in an async context. If the file contents are valid
    /// UTF-8, the Turing machine is restarted with the new code, and the code is stored in self.code.
    /// If the file contents are not valid UTF-8, a panic occurs with an "Invalid UTF-8 sequence" error message.
    ///
    /// For non-WebAssembly targets, a file dialog is spawned to select a file, sets the file filter
    /// to "TuringMachine" with a ".tm" extension, and reads the contents of the selected file using
    /// std::fs::read_to_string. If the file contents are successfully read, the Turing machine is
    /// restarted with the new code, and the code is stored in self.code.
    ///
    /// If the Turing machine fails to restart due to an error, the error is stored in self.error, and
    /// the Turing machine state remains unchanged. If no file is selected or the file dialog operation
    /// fails, the method does nothing.
    #[cfg(target_family = "wasm")]
    async fn load_file() -> Option<String> {
        let res = rfd::AsyncFileDialog::new()
            .add_filter("TuringMachine", &["tm"])
            .pick_file()
            .await;

        match res {
            Some(file) => {
                console_log!("The file {:?} has been loaded, reading...", file);

                let data: Vec<u8> = file.read().await;

                match String::from_utf8(data) {
                    Ok(s) => Some(s),
                    Err(e) => {
                        console_err!("Error reading file: {:?}", e);
                        None
                    }
                }
            }
            None => {
                console_err!("Error selecting file");
                None
            }
        }
    }

    #[cfg(not(target_family = "wasm"))]
    fn load_file(&mut self) {
        let path = std::env::current_dir().unwrap();

        let res = rfd::FileDialog::new()
            .add_filter("TuringMachine", &["tm"])
            .set_directory(path)
            .pick_file();

        if let Some(file) = res {
            let unparsed_file = std::fs::read_to_string(file).expect("cannot read file");
            self.tm = match self.tm.restart(&unparsed_file) {
                Ok(t) => {
                    self.error = None;
                    t
                }
                Err(e) => {
                    self.error = Some(e);
                    self.tm.clone()
                }
            };
            self.code = unparsed_file;
        }
    }

    /// Handles the display and behavior of various windows like the About window, Debugger window, Infinite Loop window, and the Book window.
    ///
    /// # Arguments
    ///
    /// * ctx - An egui::Context object required for creating and displaying UI components.
    /// * lang - A string representing the language used for displaying text in the UI.
    fn handle_windows(&mut self, ctx: &egui::Context, lang: &str) {
        #[cfg(not(target_family = "wasm"))]
        if self.config.times_opened >= 2 && !self.config.served_survey {
            egui::Window::new(t!("window.title.survey", lang))
                .collapsible(false)
                .default_pos([
                    ctx.screen_rect().width() / 2.0 - 400.0,
                    ctx.screen_rect().height() / 2.0 - 200.0,
                ])
                .show(ctx, |ui| {
                    ui.label(t!("lbl.window.survey", lang));

                    if ui.link(t!("window.link.survey", lang)).clicked() {
                        webbrowser::open(
                            "https://next.coldboard.net/apps/forms/s/5HYog9PptWdx458y6F2LJNGL",
                        )
                        .unwrap();
                        self.config.survey_served();
                    }

                    ui.label(t!("lbl.window.survey2", lang));
                });
        }

        if let Some(about) = &self.about_window {
            if !about.show(ctx) {
                self.about_window = None;
            } else if let Some(about) = &mut self.about_window {
                about.set_lang(lang);
            }
        }
        if let Some(debug) = &self.debug_window {
            if !debug.show(ctx) {
                self.debug_window = None;
            } else if let Some(debug) = &mut self.debug_window {
                debug.set_lang(lang);
                if self.error.is_none() {
                    debug.set_values(self.tm.tape_values(), self.tm.tape_value());
                }
            }
        }

        if let Some(inf_loop) = &self.infinite_loop_window {
            if !inf_loop.show(ctx) {
                self.infinite_loop_window = None;
                self.tm.paused = false;
            } else if let Some(inf_loop) = &mut self.infinite_loop_window {
                inf_loop.set_lang(lang);
                self.tm.paused = true;
                self.tm.reset_frequencies();
            }
        }

        if let Some(book) = self.book_window.as_mut() {
            let (active, code) = book.show(ctx);

            if let Some(c) = code {
                self.restart(&c);
                self.debug_window = Some(Box::new(DebugWindow::new(
                    lang,
                    None,
                    None,
                    Some(egui::Pos2::new(0.0, 100.0)),
                )));
                self.book_window = None;
            } else if !active {
                self.book_window = None;
            }
        }

        if let Some(editor) = self.workbook_editor_window.as_mut() {
            if !editor.show(ctx) {
                self.workbook_editor_window = None;
            }
        }

        if let Some(composition) = &self.composition_help_window {
            if !composition.show(ctx) {
                self.composition_help_window = None;
            }
        }
    }

    /// Draws the top panel containing the menu with options for file handling, debugger, exercises, language, and about information.
    ///
    /// # Arguments
    ///
    /// * ctx - An egui::Context object required for creating and displaying UI components.
    /// * lang - A string representing the language used for displaying text in the UI
    fn draw_top_panel(&mut self, ctx: &egui::Context, lang: &str) {
        egui::TopBottomPanel::top("header")
            .default_height(if is_mobile(ctx) { 50.0 } else { 35.0 })
            .show(ctx, |ui| {
                egui::ScrollArea::horizontal()
                    .max_width(ctx.screen_rect().width())
                    .show(ui, |ui| {
                        ui.horizontal_centered(|ui| {
                            ui.menu_button(t!("menu.file", lang), |ui| {
                                if ui
                                    .add(egui::Button::new("Open").shortcut_text("Ctrl + O"))
                                    .clicked()
                                {
                                    #[cfg(target_family = "wasm")]
                                    {
                                        self.file_request_future = Some(
                                            poll_promise::Promise::spawn_local(Self::load_file()),
                                        );
                                    }

                                    #[cfg(not(target_family = "wasm"))]
                                    self.load_file();
                                }

                                if ui
                                    .add(egui::Button::new("Save").shortcut_text("Ctrl + S"))
                                    .clicked()
                                {
                                    #[cfg(not(target_family = "wasm"))]
                                    self.save_file();

                                    #[cfg(target_family = "wasm")]
                                    downloadToFile(&self.code, "my-turing-program.tm");
                                }

                                if ui
                                    .add(
                                        egui::Button::new("Save as...")
                                            .shortcut_text("Ctrl + Shift + S"),
                                    )
                                    .clicked()
                                {
                                    #[cfg(not(target_family = "wasm"))]
                                    self.save_file_as();

                                    #[cfg(target_family = "wasm")]
                                    downloadToFile(&self.code, "my-turing-program.tm");
                                }

                                #[cfg(not(target_family = "wasm"))]
                                ui.add_enabled_ui(self.file.is_some(), |ui| {
                                    let prev = self.autosave;
                                    ui.checkbox(&mut self.autosave, "Autosave");

                                    if prev != self.autosave {
                                        self.config.set_autosave_disabled(self.autosave);
                                    }
                                });
                            });

                            if ui.button(t!("menu.debugger", lang)).clicked() && self.debug_window.is_none() {
                                self.debug_window = Some(Box::new(DebugWindow::new(
                                    lang,
                                    Some(self.tm.tape_values()),
                                    Some(self.tm.tape_value()),
                                    Some(egui::Pos2::new(0.0, 100.0)),
                                )));
                            }

                            if cfg!(feature = "teacher") {
                                ui.menu_button(t!("menu.exercises", lang), |ui| {
                                    if ui.button(t!("menu.exercises", lang)).clicked()
                                        && self.book_window.is_none()
                                    {
                                        self.book_window =
                                            Some(Box::new(WorkbookWindow::new(&self.get_lang())));
                                    }

                                    if ui.button(t!("menu.exercises.editor", lang)).clicked()
                                        && self.workbook_editor_window.is_none()
                                    {
                                        self.workbook_editor_window = Some(Box::new(
                                            WorkbookEditorWindow::new(&self.get_lang()),
                                        ));
                                    }
                                });
                            } else if ui.button(t!("menu.exercises", lang)).clicked()
                                && self.book_window.is_none()
                            {
                                self.book_window =
                                    Some(Box::new(WorkbookWindow::new(&self.get_lang())));
                            }

                            ui.menu_button(t!("menu.language", lang), |ui| {
                                #[cfg(target_family = "wasm")]
                                {
                                    ui.radio_value(
                                        &mut self.lang,
                                        Language::English,
                                        t!("lang.en", lang),
                                    );
                                    ui.radio_value::<Language>(
                                        &mut self.lang,
                                        Language::Spanish,
                                        t!("lang.es", lang),
                                    );
                                }
                                #[cfg(not(target_family = "wasm"))]
                                {
                                    ui.radio_value(
                                        &mut self.config.language,
                                        Language::English,
                                        t!("lang.en", lang),
                                    );
                                    ui.radio_value::<Language>(
                                        &mut self.config.language,
                                        Language::Spanish,
                                        t!("lang.es", lang),
                                    );

                                    if self.config.language.to_string() != lang {
                                        self.config.save();
                                    }
                                }
                            });

                            ui.menu_button(t!("menu.about", lang), |ui| {
                                if ui.button(t!("menu.about", lang)).clicked() {
                                    self.about_window = Some(Box::new(AboutWindow::new(
                                        lang,
                                        Some(egui::Pos2::new(150.0, 100.0)),
                                    )));
                                }

                                if ui.link(t!("menu.repository", lang)).clicked() {
                                    webbrowser::open(
                                        "https://github.com/margual56/turing-machine-2.0",
                                    )
                                    .unwrap();
                                }

                                if ui.link(t!("window.title.survey", lang)).clicked() {
                                    webbrowser::open(
                                        "https://next.coldboard.net/apps/forms/s/5HYog9PptWdx458y6F2LJNGL",
                                        )
                                        .unwrap();

                                    #[cfg(not(target_family = "wasm"))]
                                    self.config.survey_served();
                                }
                            });
                        });
                    });
            });
    }

    /// Draws the side panel containing the file open/save buttons, compile button, and code editor.
    /// It also handles autosaving and displays a "Saved file" feedback popup if applicable.
    ///
    /// # Returns
    ///
    /// The x-coordinate of the right side of the side panel.
    ///
    /// # Arguments
    ///
    /// * ctx - An egui::Context object required for creating and displaying UI components.
    /// * lang - A string representing the language used for displaying text in the UI.
    /// * editor_focused - A mutable reference to a boolean indicating whether the code editor is currently focused.
    fn draw_side_panel(
        &mut self,
        ctx: &egui::Context,
        lang: &str,
        editor_focused: &mut bool,
    ) -> f32 {
        let contents = |ui: &mut egui::Ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    let spacer = 10.0;

                    if ui
                        .add(egui::Button::new(t!("btn.open_file", lang)).min_size(
                            ui.available_size() / 2.0 - egui::Vec2::new(spacer / 2.0, 0.0),
                        ))
                        .clicked()
                    {
                        #[cfg(target_family = "wasm")]
                        {
                            self.file_request_future =
                                Some(poll_promise::Promise::spawn_local(Self::load_file()));
                        }

                        #[cfg(not(target_family = "wasm"))]
                        self.load_file();
                    }

                    ui.add_space(spacer);

                    if ui
                        .add(
                            egui::Button::new(t!("btn.save_file", lang))
                                .min_size(ui.available_size()),
                        )
                        .clicked()
                    {
                        #[cfg(not(target_family = "wasm"))]
                        self.save_file();

                        #[cfg(target_family = "wasm")]
                        downloadToFile(&self.code, "my-turing-program.tm");
                    }
                });

                if ui
                    .button(egui::RichText::new(t!("btn.compile", lang)).strong())
                    .clicked()
                {
                    self.tm = match self.tm.restart(&self.code) {
                        Ok(t) => {
                            self.error = None;
                            t
                        }
                        Err(e) => {
                            self.error = Some(e);
                            self.tm.clone()
                        }
                    };
                }

                if self.tm.uses_libraries() {
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .id_source("Library help scroll area")
                        .max_height(ui.available_height() / 2.0)
                        .show(ui, |ui| {
                            for lib in self.tm.libraries() {
                                ui.collapsing(String::from(lib.name.clone()), |ui| {
                                    egui::ScrollArea::horizontal().show(ui, |ui| {
                                        ui.horizontal(|ui| {
                                            ui.label(t!("lbl.state.initial", lang) + ":");
                                            ui.label(
                                                egui::RichText::new(lib.initial_state.clone())
                                                    .strong(),
                                            );
                                        });
                                        ui.add_space(5.0);

                                        ui.horizontal(|ui| {
                                            ui.label(t!("lbl.state.final", lang) + ":");
                                            ui.label(
                                                egui::RichText::new(lib.final_state.clone())
                                                    .strong(),
                                            );
                                        });
                                        ui.add_space(5.0);

                                        ui.horizontal(|ui| {
                                            ui.label(t!("lbl.state.used", lang) + ":");
                                            ui.label(
                                                egui::RichText::new(lib.used_states.join(", "))
                                                    .strong(),
                                            );
                                        });
                                    });
                                })
                                .header_response
                                .on_hover_text_at_pointer(lib.description.clone());
                            }
                        });
                }

                egui::ScrollArea::vertical()
                    .max_height(ui.available_height() - 50.0)
                    .show(ui, |my_ui: &mut Ui| {
                        let editor = TextEdit::multiline(&mut self.code)
                            .code_editor()
                            .desired_width(0.0)
                            .show(my_ui);

                        let res = editor.response;

                        // Autosave only works on desktop
                        #[cfg(not(target_family = "wasm"))]
                        if self.autosave && res.lost_focus() {
                            console_log!("Saving file");

                            self.saved_feedback = self.auto_save_file();
                        }

                        *editor_focused = res.has_focus();

                        // FIXME: Does not work because TextEdit is lacking the Sense(click)
                        // res.context_menu(|ui| {
                        //     if ui.button("Copy").clicked() {
                        //         if let Some(cursor_range) = editor.cursor_range {
                        //             let start = cursor_range.primary.ccursor.index;
                        //             let end = cursor_range.secondary.ccursor.index;

                        //             let text = &self.code[start..end];
                        //             ui.output_mut(|o| o.copied_text = String::from(text));
                        //         }
                        //     }
                        // });
                    });

                if ui.button(t!("btn.libraries", lang)).clicked() {
                    self.composition_help_window =
                        Some(Box::new(CompositionHelpWindow::new(&self.get_lang())));
                }

                if self.saved_feedback.is_some() {
                    console_log!("Drawing saved feedback popup");

                    self.draw_saved_feedback_popup(ui, ctx);
                }
            })
        };

        if is_mobile(ctx) {
            egui::Window::new(t!("header.code", lang))
                .collapsible(true)
                .default_pos(egui::pos2(0.0, 0.0))
                .constrain(true)
                .show(ctx, contents);
            0.0
        } else {
            egui::SidePanel::left("left")
                .show(ctx, contents)
                .response
                .rect
                .right()
        }
    }

    /// Draws the central panel containing the Turing machine description, sliders for tape size, animation speed,
    /// and infinite loop threshold, as well as the current output and playback controls.
    ///
    /// # Arguments
    ///
    /// * ctx - An egui::Context object required for creating and displaying UI components.
    /// * lang - A string representing the language used for displaying text in the UI.
    /// * editor_focused - A boolean indicating whether the code editor is currently focused.
    fn draw_central_panel(&mut self, ctx: &egui::Context, lang: &str, editor_focused: bool) {
        egui::CentralPanel::default().show(ctx, |main_panel| {
            main_panel.horizontal_top(|horiz| {
                horiz.vertical_centered(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        if let Some(desc) = self.tm.description() {
                            ui.label(
                                egui::RichText::new(desc)
                                    .color(egui::Color32::GOLD)
                                    .size(20.0)
                                    .underline(),
                            );
                        }

                        let mut sliders = |ui: &mut egui::Ui| {
                            let _prev_tape_size = self.tm.tape_rect_size;
                            let _prev_tape_speed = self.tm.tape_anim_speed;
                            let _prev_threshold_inf_loop = self.tm.threshold_inf_loop;

                            ui.add(
                                egui::Slider::new(&mut self.tm.tape_rect_size, 25.0..=300.0)
                                    .suffix(" px")
                                    .text(t!("lbl.tape.size", lang)),
                            )
                            .on_hover_text_at_pointer(t!("tooltip.tape.size", lang));
                            ui.add(
                                egui::Slider::new(&mut self.tm.tape_anim_speed, 0.2..=2.0)
                                    .suffix(t!("lbl.seconds", lang))
                                    .text(t!("lbl.tape.speed", lang)),
                            )
                            .on_hover_text_at_pointer(t!("tooltip.tape.duration", lang));
                            ui.add(
                                egui::Slider::new(&mut self.tm.threshold_inf_loop, 10..=2000)
                                    .suffix(t!("lbl.iterations", lang))
                                    .text(t!("lbl.tape.inf_loop", lang)),
                            )
                            .on_hover_text_at_pointer(t!("tooltip.tape.iterations", lang));

                            #[cfg(not(target_family = "wasm"))]
                            {
                                if _prev_tape_size != self.tm.tape_rect_size {
                                    self.config.set_tape_size(self.tm.tape_rect_size);
                                }

                                if _prev_tape_speed != self.tm.tape_anim_speed {
                                    self.config.set_tape_speed(self.tm.tape_anim_speed);
                                }

                                if _prev_threshold_inf_loop != self.tm.threshold_inf_loop {
                                    self.config
                                        .set_threshold_inf_loop(self.tm.threshold_inf_loop);
                                }
                            }
                        };

                        if is_mobile(ctx) {
                            ui.collapsing(t!("header.sliders", lang), |ui| {
                                egui::ScrollArea::horizontal()
                                    .max_width(ctx.screen_rect().width())
                                    .show(ui, sliders);
                            });
                        } else {
                            sliders(ui);
                        }
                    });

                    ui.separator();

                    ui.spacing();
                    ui.spacing();

                    match &self.tm.tape_value() {
                        TuringOutput::Undefined(_) => {
                            ui.label(t!("lbl.undefined", lang));
                        }
                        TuringOutput::Defined((_, out)) => {
                            ui.label(t!("lbl.current_output", out: &out.to_string(), lang));
                        }
                    }

                    ui.spacing();
                    ui.spacing();

                    ui.vertical_centered(|ui| {
                        let mut text = t!("lbl.pause", lang);
                        if self.tm.finished() {
                            ui.label(t!("lbl.finished", lang));
                            text = t!("lbl.restart", lang)
                        } else if self.tm.paused {
                            ui.label(t!("lbl.paused", lang));
                            text = t!("lbl.resume", lang);
                        } else {
                            ui.label(t!("lbl.resumed", lang));
                        }

                        ui.horizontal(|ui| {
                            ui.add_space(ui.available_width() / 2.0 - 115.0);
                            let b = ui
                                .button(text)
                                .on_hover_text_at_pointer(t!("tooltip.button.playpause", lang));

                            if (b.clicked()
                                || ui.input_mut(|i| {
                                    i.consume_key(egui::Modifiers::NONE, egui::Key::Space)
                                }))
                                && !editor_focused
                            {
                                if self.tm.finished() {
                                    self.tm = self.tm.restart(&self.code).unwrap();
                                } else {
                                    self.tm.paused = !self.tm.paused;
                                }
                            }
                            if self.process_turing_controls(ui, ctx, editor_focused, lang) {
                                ctx.request_repaint();
                                if self.tm.is_inf_loop() {
                                    console_warn!("Infinite loop detected!");

                                    self.infinite_loop_window =
                                        Some(Box::new(InfiniteLoopWindow::new(&self.get_lang())));
                                    self.tm.paused = true;
                                }
                            }
                        });
                    });

                    self.tm.lang = self.get_lang();
                    ui.add(&mut self.tm);

                    if let Some(e) = &self.error {
                        Self::handle_error(ui, ctx, e);
                    }
                });
            });
        });
    }
}

impl<'a> eframe::App for MyApp<'static> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        let lang = self.get_lang();
        let mut editor_focused = false;

        #[cfg(target_family = "wasm")]
        if let Some(file_async) = &self.file_request_future {
            if let Some(file_result) = file_async.ready() {
                if let Some(new_code) = file_result {
                    self.tm = match self.tm.restart(new_code) {
                        Ok(t) => {
                            self.error = None;
                            t
                        }
                        Err(e) => {
                            self.error = Some(e);
                            self.tm.clone()
                        }
                    };
                    self.code = String::from(new_code);
                }

                self.file_request_future = None;
            }
        }

        ctx.input_mut(|i| {
            // Check for keyboard shortcuts
            if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::S,
            )) {
                // Ctrl+S
                console_log!("Saving...");

                #[cfg(not(target_family = "wasm"))]
                {
                    self.save_file();
                }
                #[cfg(target_family = "wasm")]
                {
                    downloadToFile(&self.code, "my-turing-program.tm");
                }
            } else if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::O,
            )) {
                // Ctrl+O
                console_log!("Opening...");

                #[cfg(target_family = "wasm")]
                {
                    self.file_request_future =
                        Some(poll_promise::Promise::spawn_local(Self::load_file()));
                }

                #[cfg(not(target_family = "wasm"))]
                {
                    self.load_file();
                }
            } else if i.modifiers.shift
                && i.consume_shortcut(&egui::KeyboardShortcut::new(
                    egui::Modifiers::CTRL,
                    egui::Key::S,
                ))
            {
                // Ctrl+Shift+S
                console_log!("Opening...");

                #[cfg(target_family = "wasm")]
                {
                    self.file_request_future =
                        Some(poll_promise::Promise::spawn_local(Self::load_file()));
                }

                #[cfg(not(target_family = "wasm"))]
                {
                    self.save_file_as();
                }
            } else if i.consume_shortcut(&egui::KeyboardShortcut::new(
                egui::Modifiers::COMMAND,
                egui::Key::R,
            )) {
                // Ctrl+R
                console_log!("Restarting...");

                self.tm = self.tm.restart(&self.code).unwrap();
            }
        });

        self.handle_windows(ctx, &lang);

        self.draw_top_panel(ctx, &lang);

        self.tm.left = self.draw_side_panel(ctx, &lang, &mut editor_focused);

        self.draw_central_panel(ctx, &lang, editor_focused);
    }
}
