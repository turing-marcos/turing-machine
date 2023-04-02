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

const DEFAULT_CODE: &str = include_str!("../Examples/Example1.tm");

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

    /// Saves the current code to the file,
    /// and returns a time if the file was saved, or None if it wasn't
    fn save_file(&self) -> Option<Instant> {
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
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let lang = self.get_lang();
        let mut editor_focused = false;

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

        egui::TopBottomPanel::top("header")
            .default_height(35.0)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Open").clicked() {}

                        if ui.button("Save").clicked() {}

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

        self.tm.left = egui::SidePanel::left("left")
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
                            #[cfg(target_family = "wasm")]
                            {
                                // Spawn dialog on main thread
                                let task = rfd::AsyncFileDialog::new().pick_file();

                                // Await somewhere else
                                wasm_bindgen_futures::spawn_local(async move {
                                    let file = task.await;

                                    if let Some(file) = file {
                                        // If you care about wasm support you just read() the file
                                        let buffer = file.read().await;
                                        match String::from_utf8(buffer) {
                                            Ok(s) => {
                                                self.tm = match self.tm.restart(&s) {
                                                    Ok(t) => {
                                                        self.error = None;
                                                        t
                                                    }
                                                    Err(e) => {
                                                        self.error = Some(e);
                                                        self.tm.clone()
                                                    }
                                                };
                                                self.code = s;
                                            }
                                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                                        }
                                    }
                                });
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
                                        let unparsed_file = std::fs::read_to_string(&file[0])
                                            .expect("cannot read file");
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

                        ui.add_space(spacer);

                        if ui
                            .add(
                                egui::Button::new(t!("btn.save_file", lang))
                                    .min_size(ui.available_size()),
                            )
                            .clicked()
                        {
                            #[cfg(target_family = "wasm")]
                            {
                                // Spawn dialog on main thread
                                let task = rfd::AsyncFileDialog::new().save_file();

                                // Await somewhere else
                                wasm_bindgen_futures::spawn_local(async move {
                                    let file = task.await;

                                    if let Some(file) = file {
                                        // If you care about wasm support you just read() the file
                                        file.write_all(self.code.as_bytes()).await;
                                    }
                                });
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
                                    std::fs::write(&f, self.code.as_bytes())
                                        .expect("cannot write file");
                                } else {
                                    error!("Cannot save file");
                                }
                            }
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

                        if res.lost_focus() {
                            debug!("Saving file");
                            self.saved_feedback = self.save_file();
                        }

                        editor_focused = res.has_focus();
                    });

                    if self.saved_feedback.is_some() {
                        debug!("Drawing saved feedback popup");
                        self.draw_saved_feedback_popup(ui, ctx);
                    }
                })
            })
            .response
            .rect
            .right();

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
