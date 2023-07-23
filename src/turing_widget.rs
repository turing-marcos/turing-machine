use eframe::egui::{self, widgets::Widget};
use eframe::emath::Align2;
use eframe::epaint::{Color32, FontFamily, FontId, Pos2, Rect, Rounding, Stroke, Vec2};
use internationalization::t;

use log::warn;
use turing_lib::{CompilerError, CompilerWarning, Library, TuringMachine, TuringOutput};

const STROKE_WIDTH: f32 = 3f32;

#[derive(Debug, Clone)]
/// A widget that displays a Turing machine
pub struct TuringWidget {
    stroke_width: f32,
    pub tape_rect_size: f32,
    pub font_id: FontId,
    pub offset: f32,
    pub paused: bool,
    pub tape_anim_speed: f32,
    pub left: f32,
    pub threshold_inf_loop: usize, // Threshold for infinite loop detection
    tri_color: Color32,
    tri_stroke_wid: f32,
    tri_stroke: Stroke,
    tri_size: f32,
    tm: TuringMachine,
    warnings: Vec<CompilerWarning>,
    pub lang: String,
}

impl TuringWidget {
    /// Creates a new TuringWidget from a TuringMachine
    pub fn new(tm: TuringMachine, warnings: Vec<CompilerWarning>) -> Self {
        let tri_color = Color32::from_rgb(148, 73, 141);
        let tri_stroke_wid: f32 = 10.0;
        let tri_stroke = Stroke::new(tri_stroke_wid, tri_color);
        let tri_size: f32 = 100.0;

        Self {
            stroke_width: STROKE_WIDTH,
            offset: 0.0,
            tape_rect_size: 100.0,
            tape_anim_speed: 1.0,
            font_id: FontId::new(30f32, FontFamily::Monospace),
            paused: true,
            left: 300.0,
            threshold_inf_loop: 100,
            tri_color,
            tri_stroke_wid,
            tri_stroke,
            tri_size,
            tm,
            warnings,
            lang: "en".to_string(),
        }
    }

    /// Restarts the turing machine with the given code
    pub fn restart(&self, code: &str) -> Result<Self, CompilerError> {
        let (tm, warnings) = match TuringMachine::new(code) {
            Ok((t, warnings)) => {
                for w in &warnings {
                    warn!("Compiler warning: {:?}", w);
                }
                (t, warnings)
            }
            Err(e) => return Err(e),
        };

        Ok(Self {
            stroke_width: STROKE_WIDTH,
            offset: 0.0,
            tape_rect_size: self.tape_rect_size,
            tape_anim_speed: self.tape_anim_speed,
            font_id: self.font_id.clone(),
            paused: self.paused,
            left: self.left,
            threshold_inf_loop: self.threshold_inf_loop,
            tri_color: self.tri_color,
            tri_stroke_wid: self.tri_stroke_wid,
            tri_stroke: self.tri_stroke,
            tri_size: self.tri_size,
            tm,
            warnings,
            lang: self.lang.clone(),
        })
    }

    /// Calculate a step of the Turing machine
    pub fn step(&mut self) -> f32 {
        let prev = self.tm.tape_position;
        self.tm.step();
        self.offset = self.tm.tape_position as f32 - prev as f32;

        if self.tm.finished()
            || self.tm.is_undefined()
            || self.tm.is_infinite_loop(self.threshold_inf_loop)
        {
            self.paused = true;
        }

        return self.offset;
    }

    /// Returns the current tape value
    pub fn tape_value(&self) -> TuringOutput {
        self.tm.tape_value()
    }

    /// Returns the current tape length
    pub fn len(&self) -> usize {
        self.tm.tape.len()
    }

    /// Returns the description of the Turing machine if it exists
    /// (i.e. the triple comment at the top of the code)
    pub fn description(&self) -> Option<String> {
        self.tm.description.clone()
    }

    /// Returns the current code
    pub fn code(&self) -> &str {
        &self.tm.code
    }

    /// Returns the current warnings
    pub fn warnings(&self) -> &Vec<CompilerWarning> {
        &self.warnings
    }

    /// Returns the composed libraries
    pub fn libraries(&self) -> &Vec<Library> {
        &self.tm.composed_libs
    }

    pub fn uses_libraries(&self) -> bool {
        !self.tm.composed_libs.is_empty()
    }

    /// Returns the current values of the tape converted to strings
    pub fn tape_values(&self) -> Vec<String> {
        self.tm
            .values()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
    }

    /// Returns whether the Turing machine has finished
    pub fn finished(&self) -> bool {
        self.tm.finished()
    }

    /// Reset the frequencies of the Turing machine
    pub fn reset_frequencies(&mut self) {
        self.tm.reset_frequencies();
    }

    /// Returns true if the Turing machine is in an infinite loop
    pub fn is_inf_loop(&self) -> bool {
        self.tm.is_infinite_loop(self.threshold_inf_loop)
    }
}

impl Widget for TuringWidget {
    /// Paints the Turing machine
    fn ui(mut self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        if ui.is_rect_visible(ui.cursor()) {
            let stroke = Stroke::new(self.stroke_width, Color32::BLACK);
            let rounding = Rounding::same(10f32);
            let size = Vec2::new(self.tape_rect_size, self.tape_rect_size);
            let center =
                ui.cursor().center_top() + Vec2::new(0.0, self.tape_rect_size / 2.0 + 50.0);

            let pos = center + Vec2::new((self.offset as f32) * size.x, 0.0);

            for i in 0..(self.tm.tape.len()) {
                let position = Pos2::new(
                    pos.x + (i as f32 - self.tm.tape_position as f32) * size.x,
                    pos.y,
                );
                let mut rect = Rect::from_center_size(position, size);
                if ui.is_rect_visible(rect) {
                    if rect.min.x < self.left {
                        rect.set_left(self.left);
                    }
                    ui.painter()
                        .rect_filled(rect, rounding, Color32::LIGHT_BLUE);
                    ui.painter().rect_stroke(rect, rounding, stroke);

                    if position.x > self.left {
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

            let c1: Pos2 = center
                + Vec2::new(
                    self.tri_size / 1.75 - self.tri_stroke_wid * 2.0,
                    self.tri_size,
                );
            let c2: Pos2 = center
                + Vec2::new(
                    -self.tri_size / 1.75 + self.tri_stroke_wid * 2.0,
                    self.tri_size,
                );
            let c3: Pos2 = center + Vec2::new(0.0, self.tape_rect_size / 3.0);

            let circle_rad = self.tri_size / 2.0;
            let circle_center = center + Vec2::new(0.0, self.tri_size + 25.0);

            ui.painter().line_segment([c2, c3], self.tri_stroke);
            ui.painter().line_segment([c3, c1], self.tri_stroke);
            ui.painter()
                .circle_filled(c3, self.tri_stroke_wid / 2.0, self.tri_color);

            ui.painter()
                .circle_filled(circle_center, circle_rad, self.tri_color);
            ui.painter().text(
                circle_center,
                Align2::CENTER_CENTER,
                &self.tm.current_state,
                self.font_id.clone(),
                Color32::BLACK,
            );

            match self.tm.get_current_instruction() {
                Some(ins) => {
                    ui.painter().text(
                        center + Vec2::new(0.0, self.tri_size + 100.0),
                        Align2::CENTER_CENTER,
                        &ins,
                        self.font_id.clone(),
                        Color32::GRAY,
                    );
                }
                None => {
                    if self.tm.is_undefined() {
                        ui.painter().text(
                            center + Vec2::new(0.0, self.tri_size + 100.0),
                            Align2::CENTER_CENTER,
                            t!("err.undefined.state", self.lang),
                            self.font_id.clone(),
                            Color32::LIGHT_RED,
                        );
                        self.paused = true;
                    } else if self.tm.is_infinite_loop(self.threshold_inf_loop) {
                        ui.painter().text(
                            center + Vec2::new(0.0, self.tri_size + 100.0),
                            Align2::CENTER_CENTER,
                            "Infinite loop", //t!("err.infinite.loop", self.lang), // TODO: Translation
                            self.font_id.clone(),
                            Color32::LIGHT_RED,
                        );
                        self.paused = true;
                    }
                }
            };

            if self.tm.finished() {
                ui.painter().text(
                    center + Vec2::new(0.0, self.tri_size + 150.0),
                    Align2::CENTER_CENTER,
                    "The machine is in a final state",
                    self.font_id.clone(),
                    Color32::LIGHT_GREEN,
                );
                self.paused = true;
            }
        }

        ui.interact(
            ui.cursor(),
            egui::Id::new("turingwidget"),
            egui::Sense::focusable_noninteractive(),
        )
    }
}
