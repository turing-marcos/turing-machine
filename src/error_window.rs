#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::path::PathBuf;

use crate::Rule;
use eframe;
use eframe::egui::{self, RichText};
use eframe::epaint::Color32;

pub struct ErrorWindow {
    error: pest::error::Error<Rule>,
    file: PathBuf,
    line_msg: String,
    expected_msg: String,
    error_pos: usize,
}

impl ErrorWindow {
    pub fn new(
        error: pest::error::Error<Rule>,
        file: PathBuf,
        cc: &eframe::CreationContext<'_>,
    ) -> Self {
        let mut st = (*egui::Context::default().style()).clone();
        st.override_font_id = Some(egui::FontId::monospace(14.0));
        st.spacing.slider_width = 250.0;
        st.spacing.button_padding = egui::Vec2::new(10.0, 5.0);
        st.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);
        cc.egui_ctx.set_style(st);

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

        Self {
            error,
            file,
            line_msg,
            expected_msg,
            error_pos,
        }
    }
}

impl eframe::App for ErrorWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    RichText::new(format!(
                        "Syntax error on file {:?}",
                        self.file
                            .file_name()
                            .unwrap_or(std::ffi::OsStr::new("User input"))
                    ))
                    .color(Color32::LIGHT_RED)
                    .size(30.0)
                    .underline(),
                );
            });

            ui.spacing();
            ui.separator();
            ui.spacing();

            ui.vertical_centered_justified(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(&self.line_msg)
                            .color(Color32::YELLOW)
                            .size(20.0),
                    );
                });

                egui::ScrollArea::horizontal().show(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        egui::Frame::none()
                            .fill(Color32::DARK_GRAY)
                            .inner_margin(egui::style::Margin::same(10.0))
                            .show(ui, |ui: &mut egui::Ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(format!("{}", self.error.line()))
                                            .color(Color32::WHITE)
                                            .size(20.0),
                                    );
                                });

                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(format!(
                                            "{: ^width$}",
                                            "^",
                                            width = self.error_pos + 1
                                        ))
                                        .color(Color32::RED)
                                        .size(20.0),
                                    );

                                    ui.label(
                                        RichText::new(&self.expected_msg)
                                            .color(Color32::DARK_RED)
                                            .size(20.0),
                                    );
                                });
                            });
                    });
                });
                ui.label(
                    RichText::new(
                        "Could not initialize the tuing machine. Please fix the syntax error and try again."
                    )
                    .size(20.0)
                );
                if ui.button("Close").clicked() {
                    std::process::exit(2);
                }
            });
        });
    }
}
