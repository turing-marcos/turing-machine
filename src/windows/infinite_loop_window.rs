use super::SecondaryWindow;
use eframe::egui::{self, RichText, Window};
use internationalization::t;

#[derive(Debug, Clone)]
pub struct InfiniteLoopWindow {
    lang: String,
}

impl InfiniteLoopWindow {
    pub fn new(lang: &str) -> Self {
        Self {
            lang: String::from(lang),
        }
    }
}

impl SecondaryWindow for InfiniteLoopWindow {
    fn set_lang(&mut self, lang: &str) {
        self.lang = lang.to_string();
    }

    fn show(&self, ctx: &egui::Context) -> bool {
        let mut active = true;

        Window::new("Oops!")
            .open(&mut active)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.label(RichText::new(t!("lbl.inf_loop.title", self.lang)).heading());

                        ui.separator();

                        ui.label(
                            RichText::new(t!("lbl.inf_loop.description1", self.lang)).size(15.0),
                        );
                        ui.label(
                            RichText::new(t!("lbl.inf_loop.description2", self.lang)).size(15.0),
                        );
                    });
                });
            });

        active
    }
}
