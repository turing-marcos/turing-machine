use eframe::egui;
use internationalization::t;
use serde::{self, Deserialize, Serialize};

use crate::windows::workbook::raw_data_to_image;

use super::{exercise::Exercise, load_workbook, Workbook, MAX_IMG_SIZE};

#[derive(Serialize, Deserialize)]
pub struct BookWindow {
    lang: String,
    exercises: Workbook,
    selected: (usize, usize),
}

impl BookWindow {
    pub fn new(lang: &str) -> Self {
        let exercises: Workbook = vec![
            (
                "Chapter 1".to_string(),
                vec![
                    Exercise::new(
                        "Exercise 1",
                        Some(raw_data_to_image(
                            (703, 309),
                            include_bytes!("../../../assets/ui/exercise1/cover.png"),
                        )),
                        String::from(include_str!("../../../assets/ui/exercise1/code.tm")),
                    ),
                    Exercise::new(
                        "Exercise 2",
                        Some(raw_data_to_image(
                            (574, 228),
                            include_bytes!("../../../assets/ui/exercise2/cover.png"),
                        )),
                        String::from(include_str!("../../../assets/ui/exercise2/code.tm")),
                    ),
                ],
            ),
            //("Chapter 2".to_string(), vec![]),
        ];

        Self {
            lang: String::from(lang),
            exercises,
            selected: (0, 0),
        }
    }

    pub fn set_lang(&mut self, lang: &str) {
        self.lang = lang.to_string();
    }

    pub fn show(&mut self, ctx: &egui::Context) -> (bool, Option<String>) {
        let mut active = true;
        let mut code = None;

        egui::Window::new(t!("title.workbook", self.lang))
            .id(egui::Id::new("exercises_window"))
            .resizable(false)
            .open(&mut active)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Catalog"); //t!("title.exercises", self.lang));

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (section, (title, exercises)) in self.exercises.iter().enumerate() {
                                ui.collapsing(title, |ui| {
                                    for (i, exercise) in exercises.iter().enumerate() {
                                        if ui
                                            .add_enabled(
                                                self.selected.0 != section || self.selected.1 != i,
                                                egui::Button::new(&exercise.title),
                                            )
                                            .clicked()
                                        {
                                            self.selected.0 = section;
                                            self.selected.1 = i;
                                        }
                                    }
                                });
                            }
                        });

                        if ui.button("Load workbook").clicked() {
                            if let Some(new_exercises) = load_workbook() {
                                self.exercises = new_exercises;
                                self.selected = (0, 0);
                            }
                        }
                    });

                    ui.add(|ui: &mut egui::Ui| {
                        ui.set_min_height(350.0); // Set the minimum height to fill the available space
                        ui.separator()
                    });

                    ui.vertical_centered_justified(|ui| {
                        if let Some(img) = self.get_exercise(self.selected).get_cover() {
                            img.show_max_size(ui, MAX_IMG_SIZE);

                            // Add expandable empty space
                            ui.allocate_space(egui::Vec2::new(
                                0.0,
                                (MAX_IMG_SIZE.y - img.height() as f32) / 3.5,
                            ));
                        }

                        ui.horizontal(|ui| {
                            if ui
                                .add_enabled(self.selected.1 > 0, egui::Button::new("Previous"))
                                .clicked()
                            {
                                self.selected.1 -= 1;
                            }

                            ui.add_space(ui.available_width() - 50.0);

                            if ui
                                .add_enabled(
                                    self.selected.1 < self.exercises[self.selected.0].1.len() - 1,
                                    egui::Button::new("Next"),
                                )
                                .clicked()
                            {
                                self.selected.1 += 1;
                            }
                        });

                        if ui.button("Use this exercise").clicked() {
                            code = Some(self.get_exercise(self.selected).code.clone());
                        }
                    });
                });
            });

        (active, code)
    }

    fn get_exercise(&mut self, i: (usize, usize)) -> &mut Exercise {
        &mut self.exercises[i.0].1[i.1]
    }
}
