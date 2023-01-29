use eframe::egui::{self, RichText, Window};

use super::SecondaryWindow;
use internationalization::t;

#[derive(Debug, Clone, Default)]
pub struct AboutWindow {
    lang: String,
}

impl AboutWindow {
    pub fn new(lang: String) -> Self {
        Self { lang }
    }
}

impl SecondaryWindow for AboutWindow {
    fn set_lang(&mut self, lang: &str) {
        self.lang = lang.to_string();
    }

    fn show(&self, ctx: &egui::Context) -> bool {
        let mut active = true;

        Window::new("About")
            .open(&mut active)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        // TODO: Display the logo of the app

                        ui.label(
                            RichText::new(t!("lbl.title.name", self.lang))
                                .size(32.0)
                                .strong(),
                        );
                        ui.label(t!("lbl.version", version: env!("CARGO_PKG_VERSION"), self.lang));
                    });

                    ui.separator();
                    ui.vertical_centered_justified(|ui| {
                        ui.label(
                            RichText::new(t!("lbl.title.credits", self.lang))
                                .size(32.0)
                                .strong(),
                        );
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Marcos Gutiérrez Alonso").size(15.0).strong());
                            ui.label(t!("lbl.credits.marcos", self.lang));
                            if ui.link("margual56@gmail.com").clicked() {
                                webbrowser::open("mailto:margual56@gmail.com").unwrap();
                            }
                        });
                        ui.add_space(20.0);

                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Sezin Afsar").size(15.0).strong());
                            ui.label(t!("lbl.credits.sezin", self.lang));
                        });
                    });
                    ui.separator();
                    ui.vertical_centered_justified(|ui| {
                        ui.label(RichText::new(t!("lbl.title.about", self.lang)).size(32.0).strong());
                        ui.horizontal_centered(|ui| {
                            ui.label(t!("lbl.about.license", self.lang));
                            if ui.link(t!("lbl.about.code", self.lang)).clicked() {
                                webbrowser::open(
                                    "https://github.com/margual56/turing-machine-2.0/",
                                )
                                .unwrap();
                            }
                        });
                    });
                });
            });

        active
    }
}
