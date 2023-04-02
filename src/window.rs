use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};

use crate::{
    windows::{AboutWindow, BookWindow, DebugWindow, InfiniteLoopWindow, SecondaryWindow},
    TuringWidget,
};
use eframe;
use eframe::egui::{self, Id, RichText, TextEdit, Ui};
use eframe::epaint::Color32;
use internationalization::t;
use log::{debug, error, info, trace, warn};
use turing_lib::TuringMachine;
use turing_lib::{Rule, TuringOutput};

#[cfg(target_arch = "wasm32")]
use {
    std::sync::{Arc, Mutex},
    wasm_bindgen::prelude::wasm_bindgen,
};

const DEFAULT_CODE: &str = include_str!("../Examples/Example1.tm");

// Import the saveFile function
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/dist/.stage/save_file.js")]
extern "C" {
    fn saveFile(filename: &str, content: &str);
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Language {
    English,
    Spanish,
}

pub struct MyApp {
    code: String,
    error: Option<pest::error::Error<Rule>>,
    tm: TuringWidget,
    about_window: Option<Box<AboutWindow>>,
    debug_window: Option<Box<DebugWindow>>,
    infinite_loop_window: Option<Box<InfiniteLoopWindow>>,
    book_window: Option<Box<BookWindow>>,
    lang: Language,

    file: Option<PathBuf>,
    autosave: bool,
    saved_feedback: Option<Instant>,
}

impl MyApp {
    pub fn new(
        file: &Option<PathBuf>,
        cc: &eframe::CreationContext<'_>,
    ) -> Result<Self, pest::error::Error<Rule>> {
        let code = match file {
            Some(ref f) => {
                trace!("File provided: {:?}", file);
                let unparsed_file = fs::read_to_string(&f).expect("cannot read file");
                unparsed_file
            }
            None => {
                trace!("No file provided, opening an example");
                DEFAULT_CODE.to_string()
            }
        };

        let tm = match TuringMachine::new(&code) {
            Ok(t) => t,
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
        cc.egui_ctx.set_debug_on_hover(true);

        Ok(Self {
            code: String::from(&tm.code),
            error: None,
            tm: TuringWidget::new(tm),
            about_window: None,
            debug_window: None,
            infinite_loop_window: None,
            book_window: None,
            lang: Language::English,

            file: file.clone(),
            autosave: file.is_some(),
            saved_feedback: None,
        })
    }

    pub fn get_lang(&self) -> String {
        match self.lang {
            Language::English => String::from("en"),
            Language::Spanish => String::from("es"),
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
    fn handle_error(_ui: &mut Ui, ctx: &egui::Context, error: &pest::error::Error<Rule>) {
        let (error_pos, line_msg) = match error.line_col {
            pest::error::LineColLocation::Pos((line, col)) => {
                (col, format!("Line {}, column {}: ", line, col))
            }
            pest::error::LineColLocation::Span((line1, col1), (line2, col2)) => (
                col1,
                format!("From line {}:{} to {}:{}. Found:", line1, col1, line2, col2),
            ),
        };

        let expected_msg = match &error.variant {
            pest::error::ErrorVariant::ParsingError {
                positives,
                negatives,
            } => format!("Expected {:?}, found {:?}", positives, negatives),
            pest::error::ErrorVariant::CustomError { message } => message.clone(),
        };

        egui::TopBottomPanel::bottom("error").show(ctx, |ui| {
            egui::Frame::none()
                .fill(Color32::DARK_GRAY)
                .inner_margin(egui::style::Margin::same(10.0))
                .outer_margin(egui::style::Margin::same(0.0))
                .show(ui, |ui: &mut egui::Ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.label(RichText::new(line_msg).size(15.0).color(Color32::YELLOW));

                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{}", error.line()))
                                    .color(Color32::WHITE)
                                    .size(20.0),
                            );
                        });

                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(format!("{: ^width$}", "^", width = error_pos + 1))
                                    .color(Color32::RED)
                                    .size(20.0),
                            );

                            ui.label(
                                RichText::new(&expected_msg)
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
                ui.add_enabled(false, |ui: &mut Ui| ui.button(t!("lbl.machine.step", lang)));

                if self.tm.offset.abs() < 0.01 {
                    self.tm.offset = 0.0;
                    return false;
                } else {
                    self.tm.offset = ctx.animate_value_with_time(
                        Id::new("offset"),
                        0.0,
                        self.tm.tape_anim_speed,
                    );
                    return true;
                }
            } else if (ui
                .add_enabled(self.tm.paused, |ui: &mut Ui| {
                    ui.button(t!("lbl.machine.step", lang))
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
    fn auto_save_file(&self) -> Option<Instant> {
        if let Some(file) = &self.file {
            if let Ok(mut file) = File::create(file) {
                if let Err(e) = file.write_all(self.code.as_bytes()) {
                    error!("Error saving file: {}", e);
                } else {
                    info!("File saved");
                    return Some(Instant::now());
                }
            } else {
                error!("Error opening file \"{}\" for writing", file.display());
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
                let popup_pos: egui::Pos2;

                if ui.available_height() == 0.0 {
                    popup_pos = egui::Pos2::new(
                        cursor_pos.center().x - popup_size.x / 2.0,
                        cursor_pos.top() - 200.0,
                    );
                } else {
                    popup_pos = egui::Pos2::new(
                        cursor_pos.center().x - popup_size.x / 2.0,
                        cursor_pos.top(),
                    );
                }

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
    fn save_file(&mut self) {
        #[cfg(target_family = "wasm")]
        {
            let filename = "exercise.tm"; // Replace with your desired file name
            let content = &self.code;
            saveFile(filename, content);
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let file: Option<PathBuf> = match &self.file {
                Some(f) => Some(f.clone()),
                None => {
                    let path = std::env::current_dir().unwrap();

                    rfd::FileDialog::new()
                        .add_filter("TuringMachine", &["tm"])
                        .set_directory(&path)
                        .save_file()
                }
            };

            if let Some(f) = file {
                std::fs::write(&f, self.code.as_bytes()).expect("cannot write file");
                self.file = Some(f);

                debug!("Set auto-save file to {:?}", self.file);
            } else {
                error!("Cannot save file");
            }
        }
    }

    #[cfg(target_family = "wasm")]
    fn clone_for_load_file(&self) -> Self {
        MyApp {
            tm: self.tm.clone(),
            code: self.code.clone(),
            error: self.error.clone(),
            about_window: None,
            debug_window: None,
            infinite_loop_window: None,
            book_window: None,
            lang: self.lang.clone(),

            file: self.file.clone(),
            autosave: self.autosave.clone(),
            saved_feedback: None,
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
    fn load_file(&mut self) {
        #[cfg(target_family = "wasm")]
        {
            let task = rfd::AsyncFileDialog::new().pick_file();

            // Wrap MyApp in Arc<Mutex> with the custom clone method
            let shared_self = Arc::new(Mutex::new(self.clone_for_load_file()));

            // Await somewhere else
            let load_future = async move {
                let file = task.await;

                if let Some(f) = file {
                    // If you care about wasm support you just read() the file
                    let buffer = f.read().await;
                    match String::from_utf8(buffer) {
                        Ok(s) => {
                            let mut borrowed_self = shared_self.lock().unwrap();

                            borrowed_self.tm = match borrowed_self.tm.restart(&s) {
                                Ok(t) => {
                                    borrowed_self.error = None;
                                    t
                                }
                                Err(e) => {
                                    borrowed_self.error = Some(e);
                                    borrowed_self.tm.clone()
                                }
                            };
                            borrowed_self.code = s;
                        }
                        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                    }
                }
            };

            // Spawn future without boxing it
            wasm_bindgen_futures::spawn_local(load_future);
        }

        #[cfg(not(target_family = "wasm"))]
        {
            let path = std::env::current_dir().unwrap();

            let res = rfd::FileDialog::new()
                .add_filter("TuringMachine", &["tm"])
                .set_directory(&path)
                .pick_files();

            match res {
                Some(file) => {
                    let unparsed_file =
                        std::fs::read_to_string(&file[0]).expect("cannot read file");
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
                None => {}
            }
        }
    }

    /// Handles the display and behavior of various windows like the About window, Debugger window, Infinite Loop window, and the Book window.
    ///
    /// # Arguments
    ///
    /// * ctx - An egui::Context object required for creating and displaying UI components.
    /// * lang - A string representing the language used for displaying text in the UI.
    fn handle_windows(&mut self, ctx: &egui::Context, lang: &str) {
        if let Some(about) = &self.about_window {
            if !about.show(ctx) {
                self.about_window = None;
            } else if let Some(about) = &mut self.about_window {
                about.set_lang(&lang);
            }
        }
        if let Some(debug) = &self.debug_window {
            if !debug.show(ctx) {
                self.debug_window = None;
            } else if let Some(debug) = &mut self.debug_window {
                debug.set_lang(&lang);
                if !self.error.is_some() {
                    debug.set_values(self.tm.tape_values(), self.tm.tape_value());
                }
            }
        }

        if let Some(inf_loop) = &self.infinite_loop_window {
            if !inf_loop.show(ctx) {
                self.infinite_loop_window = None;
                self.tm.paused = false;
            } else if let Some(inf_loop) = &mut self.infinite_loop_window {
                inf_loop.set_lang(&lang);
                self.tm.paused = true;
                self.tm.reset_frequencies();
            }
        }

        if let Some(book) = self.book_window.as_mut() {
            let (active, code) = book.show(ctx);

            if let Some(c) = code {
                self.restart(&c);
                self.debug_window = Some(Box::new(DebugWindow::new(
                    &lang,
                    None,
                    None,
                    Some(egui::Pos2::new(0.0, 100.0)),
                )));
                self.book_window = None;
            } else if !active {
                self.book_window = None;
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
            .default_height(35.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open").clicked() {
                            self.load_file();
                        }

                        if ui.button("Save").clicked() {
                            self.save_file();
                        }

                        if ui.button("Save as...").clicked() {}

                        ui.checkbox(&mut self.autosave, "Autosave");
                    });

                    ui.menu_button(t!("menu.debugger", lang), |ui| {
                        let mut debug_enabled = self.debug_window.is_some();
                        ui.checkbox(&mut debug_enabled, t!("menu.debugger.activate", lang));
                        if debug_enabled {
                            if self.debug_window.is_none() {
                                self.debug_window = Some(Box::new(DebugWindow::new(
                                    &lang,
                                    Some(self.tm.tape_values()),
                                    Some(self.tm.tape_value()),
                                    Some(egui::Pos2::new(0.0, 100.0)),
                                )));
                            }
                        } else {
                            self.debug_window = None;
                        }
                    });

                    if ui.button("Exercises").clicked() {
                        self.book_window = Some(Box::new(BookWindow::new(&self.get_lang())));
                    }

                    ui.menu_button(t!("menu.language", lang), |ui| {
                        ui.radio_value(&mut self.lang, Language::English, t!("lang.en", lang));
                        ui.radio_value(&mut self.lang, Language::Spanish, t!("lang.es", lang));
                    });

                    ui.menu_button(t!("menu.about", lang), |ui| {
                        if ui.button(t!("menu.about", lang)).clicked() {
                            self.about_window = Some(Box::new(AboutWindow::new(
                                &lang,
                                Some(egui::Pos2::new(150.0, 100.0)),
                            )));
                        }

                        if ui.link(t!("menu.repository", lang)).clicked() {
                            webbrowser::open("https://github.com/margual56/turing-machine-2.0")
                                .unwrap();
                        }
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
        egui::SidePanel::left("left")
            .show(ctx, |ui| {
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
                            self.save_file();
                        }
                    });

                    if ui.button(t!("btn.compile", lang)).clicked() {
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

                    egui::ScrollArea::vertical().show(ui, |my_ui: &mut Ui| {
                        let editor = TextEdit::multiline(&mut self.code)
                            .code_editor()
                            .desired_width(0.0);

                        let res = my_ui.add(editor);

                        if self.autosave && res.lost_focus() {
                            debug!("Saving file");
                            self.saved_feedback = self.auto_save_file();
                        }

                        *editor_focused = res.has_focus().clone();
                    });

                    if self.saved_feedback.is_some() {
                        debug!("Drawing saved feedback popup");
                        self.draw_saved_feedback_popup(ui, ctx);
                    }
                })
            })
            .response
            .rect
            .right()
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

                        ui.add(
                            egui::Slider::new(&mut self.tm.tape_rect_size, 25.0..=300.0)
                                .suffix(" px")
                                .text(t!("lbl.tape.size", lang)),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.tm.tape_anim_speed, 0.2..=2.0)
                                .suffix(t!("lbl.seconds", lang))
                                .text(t!("lbl.tape.speed", lang)),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.tm.threshold_inf_loop, 10..=2000)
                                .suffix(t!("lbl.iterations", lang))
                                .text(t!("lbl.tape.inf_loop", lang)),
                        );
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
                        let b = ui.button(text);
                        //b.ctx.set_style(style);
                        if (b.clicked() || ui.input(|i| i.key_pressed(egui::Key::Space)))
                            && !editor_focused
                        {
                            if self.tm.finished() {
                                self.tm = self.tm.restart(&self.code).unwrap();
                            } else {
                                self.tm.paused = !self.tm.paused;
                            }
                        }

                        if self.process_turing_controls(ui, &ctx, editor_focused, &lang) {
                            ctx.request_repaint();
                            if self.tm.is_inf_loop() {
                                warn!("Infinite loop detected!");
                                self.infinite_loop_window =
                                    Some(Box::new(InfiniteLoopWindow::new(&self.get_lang())));
                                self.tm.paused = true;
                            }
                        }
                    });
                    ui.add(self.tm.clone());

                    if let Some(e) = &self.error {
                        Self::handle_error(ui, ctx, e);
                    }
                });
            });
        });
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let lang = self.get_lang();
        let mut editor_focused = false;

        self.handle_windows(ctx, &lang);

        self.draw_top_panel(ctx, &lang);

        self.tm.left = self.draw_side_panel(ctx, &lang, &mut editor_focused);

        self.draw_central_panel(ctx, &lang, editor_focused);
    }
}
