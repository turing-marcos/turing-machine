use eframe::egui::{self, RichText, Window};

use super::SecondaryWindow;

#[derive(Debug, Clone, Default)]
pub struct AboutWindow {}

impl SecondaryWindow for AboutWindow {
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

                        ui.label(RichText::new("Turing Machine Simulator").size(32.0).strong());
                        ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
                    });

                    ui.separator();
                    ui.vertical_centered_justified(|ui| {
                        ui.label(RichText::new("Authors & honorable mentions").size(32.0).strong());
                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Marcos Gutiérrez Alonso").size(15.0).strong());
                            ui.label("Original author and maintainer of the project");
                            if ui.link("margual56@gmail.com").clicked() {
                                webbrowser::open("mailto:margual56@gmail.com").unwrap();
                            }
                        });
                        ui.add_space(20.0);

                        ui.vertical_centered(|ui| {
                            ui.label(RichText::new("Sezin Afsar").size(15.0).strong());
                            ui.label("Supervisor of the project and a great moral support");
                        });
                    });
                    ui.separator();
                    ui.vertical_centered_justified(|ui| {
                        ui.label(RichText::new("About the application").size(32.0).strong());
                        ui.horizontal_centered(|ui| {
                            ui.label("This project is licensed under GPLv2");
                            if ui.link("Take a look at the source code").clicked() {
                                webbrowser::open("https://github.com/margual56/turing-machine-2.0/")
                                    .unwrap();
                            }
                        });
                    });
                });
            });

        active
    }
}
