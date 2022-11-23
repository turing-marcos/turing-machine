#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use crate::{TuringMachine, TuringWidget};
use eframe;
use eframe::egui::{self, Id, Ui};

pub struct MyApp {
    code: String,
    tm: TuringWidget,
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
        let mut st = (*egui::Context::default().style()).clone();
        st.override_font_id = Some(egui::FontId::monospace(14.0));
        st.spacing.slider_width = 250.0;
        st.spacing.button_padding = egui::Vec2::new(10.0, 5.0);
        st.spacing.item_spacing = egui::Vec2::new(10.0, 10.0);
        cc.egui_ctx.set_style(st);

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
        let mut editor_focused = false;
        self.tm.left = egui::SidePanel::left("left")
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Open file").clicked() {
                        let path = std::env::current_dir().unwrap();

                        let res = rfd::FileDialog::new()
                            .add_filter("TuringMachine", &["tm"])
                            .set_directory(&path)
                            .pick_files();

                        match res {
                            Some(file) => {
                                let unparsed_file =
                                    std::fs::read_to_string(&file[0]).expect("cannot read file");
                                self.tm = self.tm.restart(&unparsed_file);
                                self.code = unparsed_file;
                            }
                            None => {}
                        }
                    }
                    if ui.button("Compile and run code").clicked() {
                        self.tm = self.tm.restart(&self.code);
                    }

                    egui::ScrollArea::vertical().show(ui, |my_ui: &mut Ui| {
                        let editor = my_ui.code_editor(&mut self.code);
                        editor_focused = editor.has_focus();
                    });
                })
            })
            .response
            .rect
            .right();
        egui::CentralPanel::default().show(ctx, |main_panel| {
            main_panel.horizontal_top(|horiz| {
                horiz.vertical_centered(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.add(
                            egui::Slider::new(&mut self.tm.tape_rect_size, 20.0..=300.0)
                                .suffix(" px")
                                .text("Tape rectangle size"),
                        );
                        ui.add(
                            egui::Slider::new(&mut self.tm.tape_anim_speed, 0.2..=2.0)
                                .suffix(" seconds")
                                .text("Tape animation speed"),
                        );
                    });

                    ui.separator();

                    ui.spacing();
                    ui.spacing();

                    ui.label(format!("Current output: {}", self.tm.tape_value()));

                    ui.spacing();
                    ui.spacing();

                    ui.vertical_centered(|ui| {
                    let mut text = "Pause";
                    if self.tm.paused {
                        ui.label(
                    "The application is paused.\nTo unpause it, press the spacebar or this button:",
                        );
                        text = "Resume";
                    }else{
                        ui.label(
                            "The application is unpaused.\nTo pause it, press the spacebar or this button:",
                        );
                    }
                        let b = ui.button(text);
                        //b.ctx.set_style(style);
                        if (b.clicked()
                            || ui.input().key_pressed(egui::Key::Space))
                            && !editor_focused
                        {
                            self.tm.paused = !self.tm.paused;
                        }

                        if self.process_turing_controls(ui, &ctx, editor_focused) {
                            ctx.request_repaint();
                        }
                    });
                    ui.add(self.tm.clone());
                });
            });
        });
    }
}
