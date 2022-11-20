#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::TuringMachine;
use eframe;
use eframe::egui::{self, Id, Sense};
use eframe::emath::Align2;
use eframe::epaint::{Color32, FontFamily, FontId, Pos2, Rect, Rounding, Stroke, Vec2};
// use rfd;

const STROKE_WIDTH: f32 = 3f32;

pub struct MyApp {
    dropped_files: Vec<egui::DroppedFile>,
    // picked_path: Option<String>,
    offset: f32,
    tape_rect_size: f32,
    tape_anim_speed: f32,
    font_id: FontId,
    paused: bool,
    tm: TuringMachine,
}

impl MyApp {
    pub fn new(tm: TuringMachine, cc: &eframe::CreationContext<'_>) -> Self {
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

        Self {
            dropped_files: Vec::<egui::DroppedFile>::new(),
            // picked_path: None,
            offset: 0.0,
            tape_rect_size: 100.0,
            tape_anim_speed: 1.0,
            font_id: FontId::new(30f32, FontFamily::Monospace),
            paused: true,
            tm,
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |main_panel| {
            main_panel.horizontal_top(|horiz| {
                horiz.vertical(|ui| {
                    ui.add(
                        egui::Slider::new(&mut self.tape_rect_size, 20.0..=300.0)
                            .text("Tape rectangle size"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.tape_anim_speed, 0.1..=2.0)
                            .text("Tape animation speed (in seconds)"),
                    );

                    ui.label(format!("Current output: {}", self.tm.tape_value()));

                    if self.paused {
                        ui.label(
                    "The application is paused. To unpause it, press the spacebar or this button:",
                );
                        if ui.button("Resume").clicked() || ui.input().key_pressed(egui::Key::Space)
                        {
                            self.paused = false;
                        }
                    } else {
                        ui.label(
                    "The application is unpaused. To pause it, press the spacebar or this button:",
                );
                        if ui.button("Pause").clicked() || ui.input().key_pressed(egui::Key::Space)
                        {
                            self.paused = true;
                        }
                    }

                    if self.offset != 0.0 {
                        if self.offset.abs() < 0.01 {
                            self.offset = 0.0;
                        } else {
                            self.offset = ctx.animate_value_with_time(
                                Id::new("offset"),
                                0.0,
                                self.tape_anim_speed,
                            );
                        }
                        ctx.request_repaint();
                    } else if ui.button("Step").clicked()
                        || ui.input().key_pressed(egui::Key::ArrowRight)
                        || !self.paused
                    {
                        let prev = self.tm.tape_position;
                        self.tm.step();
                        self.offset = self.tm.tape_position as f32 - prev as f32;
                        ctx.clear_animations();
                        ctx.animate_value_with_time(
                            Id::new("offset"),
                            self.offset,
                            self.tape_anim_speed,
                        );
                    }

                    let stroke = Stroke::new(STROKE_WIDTH, Color32::BLACK);
                    let rounding = Rounding::same(10f32);
                    let size = Vec2::new(self.tape_rect_size, self.tape_rect_size);
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
                    let c3: Pos2 = center + Vec2::new(0.0, self.tape_rect_size / 3.0);

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

                //horiz.code_editor(&mut self.tm.code);
            });
        });

        // Collect dropped files:
        // if !ctx.input().raw.dropped_files.is_empty() {
        //     self.dropped_files = ctx.input().raw.dropped_files.clone();
        // }
    }
}
