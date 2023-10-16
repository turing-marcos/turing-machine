use eframe::egui::{self, RichText};
use egui_extras::{Column, TableBuilder};

use turing_lib::TuringOutput;

use super::SecondaryWindow;
use internationalization::t;

#[derive(Debug, Clone, Default)]
pub struct DebugWindow {
    lang: String,
    pub tape_values: Option<Vec<String>>,
    pub tape_value: Option<TuringOutput>,
    position: egui::Pos2,
}

impl DebugWindow {
    pub fn new(
        lang: &str,
        tape_values: Option<Vec<String>>,
        tape_value: Option<TuringOutput>,
        position: Option<egui::Pos2>,
    ) -> Self {
        Self {
            lang: String::from(lang),
            tape_values,
            tape_value,
            position: position.unwrap_or(egui::Pos2::new(100.0, 100.0)),
        }
    }

    pub fn set_values(&mut self, tape_values: Vec<String>, tape_value: TuringOutput) {
        self.tape_values = Some(tape_values);
        self.tape_value = Some(tape_value);
    }
}

impl SecondaryWindow for DebugWindow {
    // fn set_position(&mut self, pos: egui::Pos2) -> Self {
    //     self.position = pos;
    //     self
    // }

    fn set_lang(&mut self, lang: &str) {
        self.lang = lang.to_string();
    }

    fn show(&self, ctx: &egui::Context) -> bool {
        let mut active = true;

        egui::Window::new(t!("title.debug", self.lang))
            .id(egui::Id::new("debug_window"))
            .resizable(false)
            .open(&mut active)
            .default_pos(self.position)
            .show(ctx, |ui| {
                if let (Some(tape_values), Some(tape_value)) = (&self.tape_values, &self.tape_value)
                {
                    TableBuilder::new(ui)
                        .auto_shrink([true, true])
                        .striped(true)
                        .cell_layout(egui::Layout::centered_and_justified(
                            egui::Direction::LeftToRight,
                        ))
                        .columns(Column::auto(), tape_values.len() + 1)
                        .header(10.0, |mut header| {
                            for i in 0..tape_values.len() {
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
                                tape_values.iter().for_each(|v| {
                                    row.col(|ui| {
                                        ui.label(v.to_string());
                                    });
                                });

                                row.col(|ui| {
                                    ui.label(format!("{:?}", tape_value));
                                });
                            });
                        });
                } else {
                    ui.label(t!("debug.lbl.no_values", self.lang));
                }
            });

        active
    }
}
