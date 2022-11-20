use eframe::egui::widgets::Widget;
use eframe::egui::{Response, Sense, Ui, WidgetInfo, WidgetType};
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

    fn layout_in_ui(&self, ui: &mut Ui) -> Response {
        ui.allocate_rect(
            Rect::from_center_size(
                Pos2::new(ui.available_width() / 2.0, ui.available_height() / 2.0),
                Vec2::new(ui.available_width(), self.tape_rect_size * 2.0 + 100.0),
            ),
            Sense::focusable_noninteractive(),
        )
    }

    fn layout_center(&self, ui: &mut Ui) -> Pos2 {
        let big_size = Vec2::new(
            self.tape_rect_size * self.tm.tape.len() as f32,
            self.tape_rect_size * 2.0 + 200.0,
        );
        let (rect, _response) = ui.allocate_at_least(big_size, Sense::focusable_noninteractive());
        ui.layout()
            .align_size_within_rect(
                Vec2::new(
                    self.tape_rect_size * self.tm.tape.len() as f32,
                    self.tape_rect_size * 2.0 + 200.0,
                ),
                rect.shrink2(ui.spacing().button_padding),
            )
            .left_center()
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
        let response = self.layout_in_ui(ui);
        response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, "Turing Machine"));

        if ui.is_rect_visible(response.rect) {
            let stroke = Stroke::new(self.stroke_width, Color32::BLACK);
            let rounding = Rounding::same(10f32);
            let size = Vec2::new(self.tape_rect_size, self.tape_rect_size);
            let center = self.layout_center(ui) + Vec2::new(-ui.available_width(), 0.0);
            //Pos2::new(ui.available_width() / 2.0,self.layout_center(ui).y);//ui.available_height() / 2.0 + self.tape_rect_size * 2.0 + 200.0,

            let pos = center + Vec2::new((self.offset as f32) * size.x, 0.0);
            let len = self.tm.tape_position;

            for i in 0..(self.tm.tape.len()) {
                let position = Pos2::new(pos.x + (i as f32 - len as f32) * size.x, pos.y);
                let rect = Rect::from_center_size(position, size);
                if ui.is_rect_visible(rect) && rect.left() > 250.0 + self.tape_rect_size/2.0 {
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
        }
        response
    }
}
