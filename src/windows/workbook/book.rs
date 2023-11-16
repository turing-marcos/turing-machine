use eframe::egui;
use internationalization::t;

use crate::windows::workbook::raw_data_to_image;

use super::{exercise::Exercise, load_workbook, Workbook, MAX_IMG_SIZE};

#[cfg(target_family = "wasm")]
use poll_promise::Promise;

pub struct BookWindow {
    lang: String,
    exercises: Workbook,
    selected: (usize, usize),

    #[cfg(target_family = "wasm")]
    file_request_future: Option<Promise<Option<Workbook>>>,
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

            #[cfg(target_family = "wasm")]
            file_request_future: None,
        }
    }

    pub fn set_lang(&mut self, lang: &str) {
        self.lang = lang.to_string();
    }

    pub fn show(&mut self, ctx: &egui::Context) -> (bool, Option<String>) {
        let mut active = true;
        let mut code = None;

        #[cfg(target_family = "wasm")]
        if let Some(file_async) = &self.file_request_future {
            if let Some(file_result) = file_async.ready() {
                if let Some(workbook) = file_result.clone() {
                    self.exercises = workbook.to_vec();
                    self.selected = (0, 0);
                }

                self.file_request_future = None;
            }
        }

        egui::Window::new(t!("title.workbook", self.lang))
            .id(egui::Id::new("exercises_window"))
            .resizable(true)
            .open(&mut active)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.vertical(|ui| {
                        ui.heading(t!("heading.workbook.catalog", self.lang));

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

                        if ui.button(t!("btn.workbook.load", self.lang)).clicked() {
                            #[cfg(target_family = "wasm")]
                            {
                                self.file_request_future =
                                    Some(poll_promise::Promise::spawn_local(load_workbook()));
                            }

                            #[cfg(not(target_family = "wasm"))]
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

                    ui.vertical(|ui| {
                        let mut img_width = MAX_IMG_SIZE.x;

                        if let Some(img) = self.get_exercise(self.selected).get_cover() {
                            let img_size = img.show_max_size(ui, MAX_IMG_SIZE).rect;

                            img_width = img_size.width();
                        }

                        ui.horizontal(|ui| {
                            let prev_button = ui.add_enabled(
                                self.selected.1 > 0,
                                egui::Button::new(t!("btn.workbook.previous", self.lang)),
                            );

                            if prev_button.clicked() {
                                self.selected.1 -= 1;
                            }

                            ui.add_space(
                                img_width
                                    - prev_button.rect.width()
                                    - t!("btn.workbook.next", self.lang).len() as f32 * 10.0,
                            );

                            if ui
                                .add_enabled(
                                    self.selected.1 < self.exercises[self.selected.0].1.len() - 1,
                                    egui::Button::new(t!("btn.workbook.next", self.lang)),
                                )
                                .clicked()
                            {
                                self.selected.1 += 1;
                            }
                        });

                        ui.vertical_centered_justified(|ui| {
                            if ui.button(t!("btn.workbook.use", self.lang)).clicked() {
                                code = Some(self.get_exercise(self.selected).code.clone());
                            }
                        });
                    });
                });
            });

        (active, code)
    }

    fn get_exercise(&mut self, i: (usize, usize)) -> &mut Exercise {
        &mut self.exercises[i.0].1[i.1]
    }
}
