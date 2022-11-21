#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::{TuringMachine, TuringWidget};
use eframe;
use eframe::egui::{self, Id, Ui};
// use rfd;

pub struct MyApp {
    code: String,
    tm: TuringWidget,
}

impl MyApp {
    pub fn new(tm: TuringMachine, _cc: &eframe::CreationContext<'_>) -> Self {
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
            code: String::from(&tm.code),
            tm: TuringWidget::new(tm),
        }
    }

    fn process_turing_controls(
        &mut self,
        ui: &mut Ui,
        ctx: &egui::Context,
        editor_focused: bool,
    ) -> bool {
        ui.add_enabled_ui(!editor_focused, |ui| {
            if self.tm.offset != 0.0 {
                ui.add_enabled(false, |ui: &mut Ui| ui.button("Step"));

                if self.tm.offset.abs() < 0.01 {
                    self.tm.offset = 0.0;
                    return false;
                } else {
                    self.tm.offset = ctx.animate_value_with_time(
                        Id::new("offset"),
                        0.0,
                        self.tm.tape_anim_speed,
                    );
                    return true;
                }
            } else if (ui
                .add_enabled(self.tm.paused, |ui: &mut Ui| ui.button("Step"))
                .clicked()
                || ui.input().key_pressed(egui::Key::ArrowRight)
                || !self.tm.paused)
                && !editor_focused
            {
                ctx.clear_animations();

                let target = self.tm.step();

                ctx.animate_value_with_time(Id::new("offset"), target, self.tm.tape_anim_speed);
                return true;
            } else {
                return false;
            }
        })
        .inner
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |main_panel| {
            let mut editor_focused = false;
            main_panel.horizontal_top(|horiz| {
                horiz.vertical(|my_ui| {
                    egui::ScrollArea::vertical().show(my_ui, |my_ui: &mut Ui| {
                        let editor = my_ui.code_editor(&mut self.code);
                        editor_focused = editor.has_focus(); 
                        if my_ui.button("Compile and run code").clicked() {
                            self.tm = TuringWidget::new(TuringMachine::new(&self.code));
                        }
                    });
                });

                horiz.vertical_centered(|ui| {
                    ui.add(
                        egui::Slider::new(&mut self.tm.tape_rect_size, 20.0..=300.0)
                            .text("Tape rectangle size"),
                    );
                    ui.add(
                        egui::Slider::new(&mut self.tm.tape_anim_speed, 0.1..=2.0)
                            .text("Tape animation speed (in seconds)"),
                    );

                    ui.separator();

                    ui.label(format!("Current output: {}", self.tm.tape_value()));

                    if self.tm.paused {
                        ui.label(
                    "The application is paused. To unpause it, press the spacebar or this button:",
                );
                        if (ui.button("Resume").clicked()
                            || ui.input().key_pressed(egui::Key::Space))
                            && !editor_focused
                        {
                            self.tm.paused = false;
                        }
                    } else {
                        ui.label(
                    "The application is unpaused. To pause it, press the spacebar or this button:",
                );
                        if (ui.button("Pause").clicked()
                            || ui.input().key_pressed(egui::Key::Space))
                            && !editor_focused
                        {
                            self.tm.paused = true;
                        }
                    }

                    if self.process_turing_controls(ui, &ctx, editor_focused) {
                        ctx.request_repaint();
                    }

                    ui.add(self.tm.clone());
                });
            });
        });
    }
}
