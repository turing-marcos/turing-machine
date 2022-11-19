#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::TuringMachine;
use eframe;
use eframe::egui;
use eframe::emath::Align2;
use eframe::epaint::{Color32, FontFamily, FontId, Pos2, Rect, Rounding, Stroke, Vec2};
// use rfd;

const STROKE_WIDTH: f32 = 3f32;

pub struct MyApp {
    dropped_files: Vec<egui::DroppedFile>,
    // picked_path: Option<String>,
    offset: f32,
    font_id: FontId,
    tm: TuringMachine,
}

impl MyApp {
    pub fn new(tm: TuringMachine) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            dropped_files: Vec::<egui::DroppedFile>::new(),
            // picked_path: None,
            offset: 0.0,
            font_id: FontId::new(30f32, FontFamily::Monospace),
            tm,
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Current output: {}", self.tm.tape_value()));

            if self.offset != 0.0 {
                if self.offset.abs() < 0.01 {
                    self.offset = 0.0;
                } else {
                    self.offset /= 1.2;
                }
                ctx.request_repaint();
            } else if ui.button("Step").clicked() || ui.input().key_pressed(egui::Key::ArrowRight) {
                let prev = self.tm.tape_position;
                self.tm.step();
                self.offset = self.tm.tape_position as f32 - prev as f32;
            }

            let stroke = Stroke::new(STROKE_WIDTH, Color32::BLACK);
            let rounding = Rounding::same(10f32);
            let size = Vec2::new(150f32, 150f32);
            let center = ui.clip_rect().center();
            let pos = center + Vec2::new((self.offset as f32) * size.x, 0.0);
            let len = self.tm.tape_position;

            for i in 0..(self.tm.tape.len()) {
                let position = Pos2::new(pos.x + (i as f32 - len as f32) * size.x, pos.y);
                let rect = Rect::from_center_size(position, size);
                ui.painter()
                    .rect_filled(rect, rounding, Color32::LIGHT_BLUE);
                ui.painter().rect_stroke(rect, rounding, stroke);
                ui.painter().text(
                    position,
                    Align2::CENTER_CENTER,
                    if self.tm.tape[i] { "1" } else { "0" },
                    self.font_id.clone(),
                    Color32::BLACK,
                );
            }

            let tri_color = Color32::from_rgb(148, 73, 141);
            let tri_stroke_wid: f32 = 10.0;
            let tri_stroke = Stroke::new(tri_stroke_wid, tri_color);
            let tri_size: f32 = 100.0;
            let c1: Pos2 = center + Vec2::new(tri_size / 1.5, tri_size);
            let c2: Pos2 = center + Vec2::new(-tri_size / 1.5, tri_size);
            let c3: Pos2 = center + Vec2::new(0.0, 40.0);

            ui.painter().line_segment([c2, c3], tri_stroke);
            ui.painter().line_segment([c3, c1], tri_stroke);
            ui.painter()
                .circle_filled(c3, tri_stroke_wid / 2.0, tri_color);

            let r1: Pos2 = c1 + Vec2::new(tri_stroke_wid / 3.64, -tri_stroke_wid / 1.25);
            let r2: Pos2 = c2 + Vec2::new(-tri_stroke_wid / 3.64, -tri_stroke_wid / 1.25);
            let r3: Pos2 = r1 + Vec2::new(0.0, 75.0);
            let r4: Pos2 = r2 + Vec2::new(0.0, 75.0);

            ui.painter().rect_filled(
                Rect::from_points(&[r1, r2, r3, r4]),
                Rounding::none(),
                tri_color,
            );
            ui.painter().text(
                center + Vec2::new(0.0, tri_size + 25.0),
                Align2::CENTER_CENTER,
                &self.tm.current_state,
                self.font_id.clone(),
                Color32::BLACK,
            );
        });

        // Collect dropped files:
        if !ctx.input().raw.dropped_files.is_empty() {
            self.dropped_files = ctx.input().raw.dropped_files.clone();
        }
    }
}
