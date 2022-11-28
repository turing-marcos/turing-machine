#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::{TuringMachine, TuringWidget};
use eframe;
use eframe::egui::{self, Id, RichText, Ui};
use eframe::epaint::Color32;

pub struct MyApp {
    code: String,
    error: Option<pest::error::Error<crate::Rule>>,
    tm: TuringWidget,
}

impl MyApp {
    pub fn new(tm: TuringMachine, cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        // let mut fonts = egui::FontDefinitions::default();
        // fonts
        //     .families
        //     .entry(egui::FontFamily::Monospace)
        //     .or_default()
        //     .insert(0, String::from("Consolas"));
        // cc.egui_ctx.set_fonts(fonts);
        let mut st = (*egui::Context::default().style()).clone();
        st.override_font_id = Some(egui::FontId::monospace(14.0));
        st.spacing.slider_width = 250.0;
        st.spacing.button_padding = egui::Vec2::new(10.0, 5.0);
        st.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);
        cc.egui_ctx.set_style(st);

        Self {
            code: String::from(&tm.code),
            error: None,
            tm: TuringWidget::new(tm),
        }
    }

    fn handle_error(ui: &mut Ui, ctx: &egui::Context, error: &pest::error::Error<crate::Rule>) {
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
    ) -> bool {
        ui.add_enabled_ui(!editor_focused, |ui| {
            if self.tm.offset != 0.0 {
                ui.add_enabled(false, |ui: &mut Ui| ui.button("Step"));
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
                .add_enabled(self.tm.paused, |ui: &mut Ui| ui.button("Step"))
                .clicked()
                || ui.input().key_pressed(egui::Key::ArrowRight)
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
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut editor_focused = false;
        self.tm.left = egui::SidePanel::left("left")
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Open file").clicked() {
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
                    if ui.button("Compile and run code").clicked() {
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
                        let editor = my_ui.code_editor(&mut self.code);
                        editor_focused = editor.has_focus();
                    });
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
                            ui.label(egui::RichText::new(desc).color(egui::Color32::GOLD).size(20.0).underline());
                            ui.separator();
                        }

                        ui.add(
                            egui::Slider::new(&mut self.tm.tape_rect_size, 20.0..=300.0)
                                .suffix(" px")
                                .text("Tape rectangle size"),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.tm.tape_anim_speed, 0.2..=2.0)
                                .suffix(" seconds")
                                .text("Tape animation speed"),
                        );
                    });

                    ui.separator();

                    ui.spacing();
                    ui.spacing();

                    ui.label(format!("Current output: {}", self.tm.tape_value()));

                    ui.spacing();
                    ui.spacing();

                    ui.vertical_centered(|ui| {
                    let mut text = "Pause";
                    if self.tm.paused {
                        ui.label(
                    "The application is paused.\nTo unpause it, press the spacebar or this button:",
                        );
                        text = "Resume";
                    }else{
                        ui.label(
                            "The application is unpaused.\nTo pause it, press the spacebar or this button:",
                        );
                    }
                        let b = ui.button(text);
                        //b.ctx.set_style(style);
                        if (b.clicked()
                            || ui.input().key_pressed(egui::Key::Space))
                            && !editor_focused
                        {
                            self.tm.paused = !self.tm.paused;
                        }

                        if self.process_turing_controls(ui, &ctx, editor_focused) {
                            ctx.request_repaint();
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
