use eframe::{
    egui::{self, RichText, TextEdit, WidgetText},
    epaint::Color32,
};
use egui_extras::{Column, TableBuilder};
use turing_lib::LIBRARIES;

use super::SecondaryWindow;
use internationalization::t;

#[derive(Debug, Clone, Default)]
pub struct CompositionHelpWindow {
    lang: String,
}

impl CompositionHelpWindow {
    pub fn new(lang: &str) -> Self {
        Self {
            lang: String::from(lang),
        }
    }
}

impl SecondaryWindow for CompositionHelpWindow {
    fn set_lang(&mut self, lang: &str) {
        self.lang = lang.to_string();
    }

    fn show(&self, ctx: &egui::Context) -> bool {
        let mut active = true;

        egui::Window::new(t!("title.composition", self.lang))
            .id(egui::Id::new("composition_help_window"))
            .resizable(true)
            .default_size([1000.0, 300.0])
            .open(&mut active)
            .show(ctx, |ui| {
                egui::ScrollArea::horizontal()
                    .auto_shrink([true, true])
                    .show(ui, |ui| {
                        TableBuilder::new(ui)
                            .auto_shrink([true, true])
                            .striped(true)
                            //.cell_layout(egui::Layout::centered_and_justified(
                            //    egui::Direction::LeftToRight,
                            //))
                            .columns(Column::auto(), 4)
                            .column(Column::auto().clip(true))
                            .header(20.0, |mut header| {
                                header.col(|ui| {
                                    ui.label(WidgetText::RichText(RichText::new(t!("lbl.composition.name", self.lang)))
                                            .heading()
                                    )
                                    .on_hover_text_at_pointer(t!(
                                        "tooltip.composition.name",
                                        self.lang
                                    ));
                                });

                                header.col(|ui| {
                                    ui.label(
                                        RichText::new(t!("lbl.composition.description", self.lang))
                                            .heading(),
                                    );
                                });

                                header.col(|ui| {
                                    ui.label(
                                        RichText::new(t!("lbl.state.initial", self.lang)).heading(),
                                    );
                                });

                                header.col(|ui| {
                                    ui.label(
                                        RichText::new(t!("lbl.state.final", self.lang)).heading(),
                                    );
                                });

                                header.col(|ui| {
                                    ui.label(
                                        RichText::new(t!("lbl.state.used", self.lang)).heading(),
                                    );
                                });
                            })
                            .body(|mut body| {
                                for lib in &LIBRARIES {
                                    body.row(30.0, |mut row| {
                                        row.col(|ui| {
                                            ui.label(lib.name.clone());
                                        });

                                        row.col(|ui| {
                                            ui.label(lib.description.clone());
                                        });

                                        row.col(|ui| {
                                            ui.label(lib.initial_state.clone());
                                        });

                                        row.col(|ui| {
                                            ui.label(lib.final_state.clone());
                                        });

                                        row.col(|ui| {
                                            ui.horizontal(|ui| {
                                                ui.label(lib.used_states.join(", ").clone());
                                            });
                                        });
                                    });
                                }
                            });

                        ui.separator();

                        ui.label(RichText::new(t!("lbl.composition.help.txt", self.lang)))
                            .on_hover_ui_at_pointer(|ui| {
                                ui.vertical_centered(|ui| {
                                    let mut sample_code = String::from(
                                        "
                                /// a + b + 1

                                {11111011};

                                I = {q0}; F = {q3};

                                compose = {sum};

                                (q2, 1, 1, R, q3);
                                (q3, 1, 1, R, q3);
                                (q3, 0, 1, H, q3);",
                                    );

                                    ui.add_enabled(
                                        false,
                                        TextEdit::multiline(&mut sample_code).code_editor(),
                                    );

                                    ui.separator();

                                    ui.label(
                                        RichText::new(t!("lbl.composition.warning", self.lang))
                                            .italics()
                                            .color(Color32::LIGHT_YELLOW),
                                    );
                                });
                            })
                    });
            });

        active
    }
}
