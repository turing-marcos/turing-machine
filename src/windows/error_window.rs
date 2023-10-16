use std::path::PathBuf;

use eframe;
use eframe::egui::{self, RichText};
use eframe::epaint::Color32;
use internationalization::t;
use turing_lib::{CompilerError, ErrorPosition};

use crate::window::Language;

pub struct ErrorWindow {
    error: CompilerError,
    file: Option<PathBuf>,
    line_msg: String,
    expected_msg: String,
    lang: Language,
}

impl ErrorWindow {
    pub fn new(
        error: CompilerError,
        file: Option<PathBuf>,
        lang: Language,
        cc: &eframe::CreationContext<'_>,
    ) -> Self {
        let mut st = (*egui::Context::default().style()).clone();
        st.override_font_id = Some(egui::FontId::monospace(14.0));
        st.spacing.slider_width = 250.0;
        st.spacing.button_padding = egui::Vec2::new(10.0, 5.0);
        st.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);
        cc.egui_ctx.set_style(st);

        let position: ErrorPosition = error.position();

        let line_msg = match position.end {
            Some(end) => format!(
                "From line {}:{} to {}:{}. Found:",
                position.start.0, position.start.1, end.0, end.1
            ),
            None => format!("At line {}:{} Found:", position.start.0, position.start.1),
        };

        let expected_msg = error.get_message_expected();

        Self {
            error,
            file,
            line_msg,
            expected_msg,
            lang,
        }
    }

    fn lang(&self) -> &str {
        match self.lang {
            Language::English => "en",
            Language::Spanish => "es",
        }
    }
}

impl eframe::App for ErrorWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let lang = String::from(self.lang());
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.menu_button(t!("menu.language", self.lang()), |ui| {
                ui.radio_value::<Language>(&mut self.lang, Language::English, t!("lang.en", lang));
                ui.radio_value::<Language>(&mut self.lang, Language::Spanish, t!("lang.es", lang));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                let text = match self.file {
                    Some(ref file) => {
                        let filename = file
                            .file_name()
                            .unwrap_or(std::ffi::OsStr::new("User input"))
                            .to_str()
                            .unwrap();

                        t!(
                            "err.syntax",
                            file: filename, lang
                        )
                    }
                    None => t!("err.syntax.simple", lang).to_string(),
                };

                ui.label(
                    RichText::new(text)
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
                            .fill(Color32::BLACK)
                            .inner_margin(egui::style::Margin::same(10.0))
                            .show(ui, |ui: &mut egui::Ui| {
                                ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(self.error.code().to_string())
                                            .color(Color32::WHITE)
                                            .size(20.0),
                                    );
                                });

                                ui.horizontal(|ui| {
                                    let position = self.error.position();
                                    ui.label(
                                        RichText::new(format!(
                                            "{:~>width1$}{:^<width2$}{:~<width3$}",
                                            "~",
                                            "^",
                                            "~",
                                            width1 = position.start.1,
                                            width2 =
                                                position.end.unwrap_or((0, position.start.1 + 1)).1
                                                    - position.start.1,
                                            width3 = self.error.code().len()
                                                - position
                                                    .end
                                                    .unwrap_or((0, position.start.1 + 1))
                                                    .1
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
                ui.label(RichText::new(t!("err.initialization", lang)).size(20.0));
                if ui.button(t!("btn.close", lang)).clicked() {
                    std::process::exit(2);
                }
            });
        });
    }
}
