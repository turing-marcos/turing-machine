use eframe::egui::{self, widgets::Widget};
use eframe::emath::Align2;
use eframe::epaint::{Color32, FontFamily, FontId, Pos2, Rect, Rounding, Stroke, Vec2};
use internationalization::t;

#[cfg(not(target_family = "wasm"))]
use log::warn;

use turing_lib::{CompilerError, CompilerWarning, Library, TuringMachine, TuringOutput};

use crate::{console_warn, window::is_mobile};

const STROKE_WIDTH: f32 = 3f32;
const FONT_SIZE: f32 = 30f32;

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
    errors: Option<CompilerError>,
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
            errors: None,
            lang: "en".to_string(),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn set_config(&self, config: &crate::config::Config) -> Self {
        let mut new_tm = self.clone();

        new_tm.lang = config.language().to_string();
        new_tm.threshold_inf_loop = config.threshold_inf_loop();
        new_tm.tape_rect_size = config.tape_size();
        new_tm.tape_anim_speed = config.tape_speed();

        new_tm
    }

    /// Restarts the turing machine with the given code
    pub fn restart(&mut self, code: &str) -> Result<Self, CompilerError> {
        let (tm, warnings) = match TuringMachine::new(code) {
            Ok((t, warnings)) => {
                for w in &warnings {
                    #[cfg(not(target_family = "wasm"))]
                    console_warn!("Compiler warning: {:?}", w);

                    #[cfg(target_family = "wasm")]
                    console_warn!("Compiler warning: {:?}", w);
                }

                self.errors = None;

                (t, warnings)
            }
            Err(e) => {
                self.errors = Some(e.clone());
                return Err(e);
            }
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
            errors: None,
            lang: self.lang.clone(),
        })
    }

    /// Calculate a step of the Turing machine
    pub fn step(&mut self) -> f32 {
        let prev = self.tm.tape_position;
        self.tm.step();
        self.offset = self.tm.tape_position as f32 - prev as f32;

        if self.finished()
            || self.tm.is_undefined()
            || self.tm.is_infinite_loop(self.threshold_inf_loop)
        {
            self.paused = true;
        }

        self.offset
    }

    /// Returns whether the turing machine is in a final state, the current state is the same as the previous state and the current instruction is HALT
    pub fn finished(&self) -> bool {
        self.tm.finished()
            && self.tm.previous_state.clone().unwrap_or_default() == self.tm.current_state
            && match self.tm.get_current_instruction() {
                Some(ins) => {
                    println!("{:?}", ins);
                    ins.movement == turing_lib::Movement::HALT
                        && ins.from_value == ins.to_value
                        && ins.from_state == ins.to_state
                }
                None => true,
            }
    }

    /// Returns the current tape value
    pub fn tape_value(&self) -> TuringOutput {
        self.tm.tape_value()
    }

    /// Returns the current tape length
    pub fn len(&self) -> usize {
        self.tm.tape.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tm.tape.is_empty()
    }

    /// Returns the description of the Turing machine if it exists
    /// (i.e. the triple comment at the top of the code)
    pub fn description(&self) -> Option<&String> {
        if self.errors.is_some() {
            return None;
        }

        self.tm.description.as_ref()
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

    /// Reset the frequencies of the Turing machine
    pub fn reset_frequencies(&mut self) {
        self.tm.reset_frequencies();
    }

    /// Returns true if the Turing machine is in an infinite loop
    pub fn is_inf_loop(&self) -> bool {
        self.tm.is_infinite_loop(self.threshold_inf_loop)
    }
}

impl Widget for &mut TuringWidget {
    /// Paints the Turing machine
    fn ui(self, ui: &mut eframe::egui::Ui) -> eframe::egui::Response {
        let mut font_id = self.font_id.clone();
        let mut stroke_width = self.stroke_width;
        let mut tri_size = self.tri_size;

        if is_mobile(ui.ctx()) {
            font_id.size = FONT_SIZE / 2.0;
            stroke_width /= 2.0;
            tri_size /= 1.5;
        }

        if ui.is_rect_visible(ui.cursor()) {
            let stroke = Stroke::new(stroke_width, Color32::BLACK);
            let rounding = Rounding::same(10f32);
            let size = Vec2::new(self.tape_rect_size, self.tape_rect_size);
            let center = ui.cursor().center_top()
                + Vec2::new(
                    if is_mobile(ui.ctx()) {
                        -ui.available_width() / 2.0 + ui.ctx().screen_rect().width() / 2.0
                    } else {
                        0.0
                    },
                    self.tape_rect_size / 2.0 + if is_mobile(ui.ctx()) { 25.0 } else { 50.0 },
                );

            let pos = center + Vec2::new(self.offset * size.x, 0.0);

            for i in 0..(self.tm.tape.len() + 5) {
                let position = Pos2::new(
                    pos.x + (i as f32 - self.tm.tape_position as f32) * size.x,
                    pos.y,
                );
                let mut rect = Rect::from_center_size(position, size);
                if ui.is_rect_visible(rect) {
                    if rect.min.x < self.left {
                        rect.set_left(self.left);
                    } else if rect.max.x > self.left + ui.ctx().screen_rect().width() - 200.0 {
                        rect.set_right(self.left + ui.ctx().screen_rect().width() - 200.0);
                    }

                    ui.painter()
                        .rect_filled(rect, rounding, Color32::LIGHT_BLUE);
                    ui.painter().rect_stroke(rect, rounding, stroke);

                    if position.x > self.left {
                        ui.painter().text(
                            position,
                            Align2::CENTER_CENTER,
                            if self.tm.get(i).unwrap_or(false) {
                                "1"
                            } else {
                                "0"
                            },
                            self.font_id.clone(),
                            Color32::BLACK,
                        );
                    }
                } else {
                    continue;
                }
            }

            let height = if is_mobile(ui.ctx()) { 1.2 } else { 1.0 };

            let c1: Pos2 = center
                + Vec2::new(
                    height * tri_size / 1.75 - self.tri_stroke_wid * 2.0,
                    height * tri_size,
                );
            let c2: Pos2 = center
                + Vec2::new(
                    -height * tri_size / 1.75 + self.tri_stroke_wid * 2.0,
                    height * tri_size,
                );
            let c3: Pos2 = center + Vec2::new(0.0, self.tape_rect_size / 3.0);

            let circle_rad = tri_size / 2.0 + if is_mobile(ui.ctx()) { 0.0 } else { 0.5 };
            let circle_center = center + Vec2::new(0.0, tri_size + 25.0);

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
                        ins,
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
                            font_id.clone(),
                            Color32::LIGHT_RED,
                        );
                        self.paused = true;
                    } else if self.tm.is_infinite_loop(self.threshold_inf_loop) {
                        ui.painter().text(
                            center + Vec2::new(0.0, self.tri_size + 100.0),
                            Align2::CENTER_CENTER,
                            t!("err.infinite_loop", self.lang),
                            font_id.clone(),
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
                    font_id,
                    Color32::LIGHT_GREEN,
                );

                if self.finished() {
                    self.paused = true;
                }
            }
        }

        ui.interact(
            ui.cursor(),
            egui::Id::new("turingwidget"),
            egui::Sense::focusable_noninteractive(),
        )
    }
}
