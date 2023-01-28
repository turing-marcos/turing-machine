use eframe::egui::{self, RichText};
use egui_extras::{TableBuilder, Column};

use super::SecondaryWindow;

#[derive(Debug, Clone, Default)]
pub struct DebugWindow {
    pub tape_values: Vec<String>,
    pub tape_value: u32,
}

impl DebugWindow {
    pub fn new(tape_values: Vec<String>, tape_value: u32) -> Self {
        Self {
            tape_values,
            tape_value,
        }
    }

    pub fn set_values(&mut self, tape_values: Vec<String>, tape_value: u32) {
        self.tape_values = tape_values;
        self.tape_value = tape_value;
    }
}

impl SecondaryWindow for DebugWindow {
    fn show(&self, ctx: &egui::Context) -> bool {
        let mut active = true;

        egui::Window::new("Debug")
            .open(&mut active)
            .show(ctx, |ui| {
                TableBuilder::new(ui).auto_shrink([true, true])
                .striped(true)
                .cell_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight))
                .columns(Column::auto(), self.tape_values.len() +1)
                .header(10.0, |mut header| {
                    for i in 0..self.tape_values.len() {
                        header.col(|ui| {
                            ui.label(RichText::new(format!("Value {}", i)).heading());
                        });
                    }

                    header.col(|ui| {
                        ui.label(RichText::new("Result").heading());
                    });
                })
                .body(|mut body| {
                    body.row(10.0, |mut row| {
                        self.tape_values.iter().for_each(|v| {
                            row.col(|ui| {
                                ui.label(format!("{}", v));
                            });
                        });

                        row.col(|ui| {
                            ui.label(format!("{}", self.tape_value));
                        });
                    });
                });
            });

        active
    }
}