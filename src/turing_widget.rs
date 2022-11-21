use eframe::egui::{self, widgets::Widget};
use eframe::emath::Align2;
use eframe::epaint::{Color32, FontFamily, FontId, Pos2, Rect, Rounding, Stroke, Vec2};

use crate::TuringMachine;

const STROKE_WIDTH: f32 = 3f32;

#[derive(Debug, Clone)]
pub struct TuringWidget {
    stroke_width: f32,
    pub tape_rect_size: f32,
    pub font_id: FontId,
    pub offset: f32,
    pub paused: bool,
    pub tape_anim_speed: f32,
    tm: TuringMachine,
}

impl TuringWidget {
    pub fn new(tm: TuringMachine) -> Self {
        Self {
            stroke_width: STROKE_WIDTH,
            offset: 0.0,
            tape_rect_size: 100.0,
            tape_anim_speed: 1.0,
            font_id: FontId::new(30f32, FontFamily::Monospace),
            paused: true,
            tm,
        }
    }

    pub fn step(&mut self) -> f32 {
        let prev = self.tm.tape_position;
        self.tm.step();
        self.offset = self.tm.tape_position as f32 - prev as f32;
        return self.offset;
    }

    pub fn tape_value(&self) -> u32 {
        self.tm.tape_value()
    }

    pub fn code(&self) -> &str {
        &self.tm.code
    }
}

impl Widget for TuringWidget {
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        ui.painter().ctx().move_to_top(egui::LayerId::new(
            egui::Order::Background,
            egui::Id::new("main"),
        ));
        if ui.is_rect_visible(ui.cursor()) {
            let stroke = Stroke::new(self.stroke_width, Color32::BLACK);
            let rounding = Rounding::same(10f32);
            let size = Vec2::new(self.tape_rect_size, self.tape_rect_size);
            let center = Pos2::new(
                300.0 + ui.available_width() / 2.0,
                ui.available_height() / 2.0,
            );

            let pos = center + Vec2::new((self.offset as f32) * size.x, 0.0);

            for i in 0..(self.tm.tape.len()) {
                let position = Pos2::new(
                    pos.x + (i as f32 - self.tm.tape_position as f32) * size.x,
                    pos.y,
                );
                let mut rect = Rect::from_center_size(position, size);
                if ui.is_rect_visible(rect) {
                    if rect.min.x < 300.0 {
                        rect.set_left(300.0);
                    }
                    ui.painter()
                        .rect_filled(rect, rounding, Color32::LIGHT_BLUE);
                    ui.painter().rect_stroke(rect, rounding, stroke);

                    if position.x > 300.0 {
                        ui.painter().text(
                            position,
                            Align2::CENTER_CENTER,
                            if self.tm.tape[i] { "1" } else { "0" },
                            self.font_id.clone(),
                            Color32::BLACK,
                        );
                    }
                }
            }

            let tri_color = Color32::from_rgb(148, 73, 141);
            let tri_stroke_wid: f32 = 10.0;
            let tri_stroke = Stroke::new(tri_stroke_wid, tri_color);
            let tri_size: f32 = 100.0;

            let c1: Pos2 = center + Vec2::new(tri_size / 1.75 - tri_stroke_wid*2.0 , tri_size);
            let c2: Pos2 = center + Vec2::new(-tri_size / 1.75 + tri_stroke_wid*2.0 , tri_size);
            let c3: Pos2 = center + Vec2::new(0.0, self.tape_rect_size / 3.0);

            let circle_rad = tri_size/2.0;
            let circle_center = center + Vec2::new(0.0, tri_size + 25.0);

            ui.painter().line_segment([c2, c3], tri_stroke);
            ui.painter().line_segment([c3, c1], tri_stroke);
            ui.painter()
                .circle_filled(c3, tri_stroke_wid / 2.0, tri_color);

            ui.painter().circle_filled(
                circle_center,
                circle_rad,
                tri_color,
            );
            ui.painter().text(
                circle_center,
                Align2::CENTER_CENTER,
                &self.tm.current_state,
                self.font_id.clone(),
                Color32::BLACK,
            );

            let ins = match self.tm.current_instruction() {
                Some(txt) => format!("{}", txt),
                None => String::from("ERROR: No instruction matches this situation!"),
            };

            ui.painter().text(
                center + Vec2::new(0.0, tri_size + 100.0),
                Align2::CENTER_CENTER,
                &ins,
                self.font_id.clone(),
                Color32::GRAY,
            );
        }
        ui.interact(
            ui.cursor(),
            egui::Id::new("turingwidget"),
            egui::Sense::focusable_noninteractive(),
        )
    }
}
