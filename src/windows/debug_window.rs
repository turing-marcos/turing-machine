use eframe::egui::{self, RichText};
use egui_extras::{Column, TableBuilder};

use super::SecondaryWindow;
use internationalization::t;

#[derive(Debug, Clone, Default)]
pub struct DebugWindow {
    lang: String,
    pub tape_values: Vec<String>,
    pub tape_value: u32,
}

impl DebugWindow {
    pub fn new(lang: &str, tape_values: Vec<String>, tape_value: u32) -> Self {
        Self {
            lang: String::from(lang),
            tape_values,
            tape_value,
        }
    }

    pub fn set_values(&mut self, tape_values: Vec<String>, tape_value: u32) {
        self.tape_values = tape_values;
        self.tape_value = tape_value;
    }
}

impl SecondaryWindow for DebugWindow {
    fn set_lang(&mut self, lang: &str) {
        self.lang = lang.to_string();
    }

    fn show(&self, ctx: &egui::Context) -> bool {
        let mut active = true;

        egui::Window::new(t!("title.debug", self.lang))
            .resizable(false)
            .open(&mut active)
            .show(ctx, |ui| {
                TableBuilder::new(ui)
                    .auto_shrink([true, true])
                    .striped(true)
                    .cell_layout(egui::Layout::centered_and_justified(
                        egui::Direction::LeftToRight,
                    ))
                    .columns(Column::auto(), self.tape_values.len() + 1)
                    .header(10.0, |mut header| {
                        for i in 0..self.tape_values.len() {
                            header.col(|ui| {
                                ui.label(
                                    RichText::new(t!(
                                        "lbl.value",
                                        val: &usize::to_string(&i),
                                        self.lang
                                    ))
                                    .heading(),
                                );
                            });
                        }

                        header.col(|ui| {
                            ui.label(RichText::new(t!("lbl.result", self.lang)).heading());
                        });
                    })
                    .body(|mut body| {
                        body.row(10.0, |mut row| {
                            self.tape_values.iter().for_each(|v| {
                                row.col(|ui| {
                                    ui.label(format!("{}", v));
                                });
                            });

                            row.col(|ui| {
                                ui.label(format!("{}", self.tape_value));
                            });
                        });
                    });
            });

        active
    }
}
