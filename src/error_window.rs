#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::Rule;
use eframe;
use eframe::egui;

pub struct ErrorWindow {
    error: pest::error::Error<Rule>,
}

impl ErrorWindow {
    pub fn new(e: pest::error::Error<Rule>, cc: &eframe::CreationContext<'_>) -> Self {
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

        Self { error: e }
    }
}

impl eframe::App for ErrorWindow {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    egui::RichText::new("There was an error parsing the file!")
                        .color(egui::Color32::LIGHT_RED)
                        .size(30.0)
                        .underline(),
                );
            });
        });
    }
}
