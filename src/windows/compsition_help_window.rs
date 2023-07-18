use eframe::egui::{self, RichText};
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

        egui::Window::new("List of composition libraries available")
            .id(egui::Id::new("composition_help_window"))
            .resizable(false)
            .open(&mut active)
            .show(ctx, |ui| {
                TableBuilder::new(ui)
                    .auto_shrink([true, true])
                    .striped(true)
                    .resizable(true)
                    .cell_layout(egui::Layout::centered_and_justified(
                        egui::Direction::LeftToRight,
                    ))
                    .columns(Column::auto(), 4)
                    .column(Column::auto().clip(true).resizable(true))
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.label(
                                RichText::new("Library name") // TODO: Translate
                                    .heading(),
                            )
                            .on_hover_text_at_pointer("The name to write to import it");
                            // TODO: Translate
                        });

                        header.col(|ui| {
                            ui.label(RichText::new("Description").heading()); // TODO: Translate
                        });

                        header.col(|ui| {
                            ui.label(RichText::new("Initial state").heading()); // TODO: Translate
                        });

                        header.col(|ui| {
                            ui.label(RichText::new("Final state").heading()); // TODO: Translate
                        });

                        header.col(|ui| {
                            ui.label(RichText::new("Used states").heading()); // TODO: Translate
                        });
                    })
                    .body(|mut body| {
                        for lib in &LIBRARIES {
                            body.row(20.0, |mut row| {
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
                                    egui::ScrollArea::horizontal().auto_shrink([true, true]).id_source(String::from(lib.name.clone()) + "_scroll").show(ui, |ui| {
                                        ui.label(lib.used_states.join(", ").clone()); // (String::from(lib.name.clone()) + "_used_states")
                                    });
                                });
                            });
                        }
                    });
            });

        active
    }
}
