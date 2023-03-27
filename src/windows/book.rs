use eframe::{egui, epaint::Vec2};
use egui_extras::RetainedImage;
use internationalization::t;

struct Exercise {
    image: RetainedImage,
    title: String,
    code: String,
}

impl Exercise {
    pub fn new(title: &str, img: &[u8], code: String) -> Self {
        Self {
            image: RetainedImage::from_image_bytes(title, img).unwrap(),
            title: String::from(title),
            code: String::from(code),
        }
    }
}

pub struct BookWindow {
    lang: String,
    exercises: Vec<(String, Vec<Exercise>)>,
    selected: (usize, usize),
}

impl BookWindow {
    pub fn new(lang: &str) -> Self {
        let exercises: Vec<(String, Vec<Exercise>)> = vec![
            (
                "Chapter 1".to_string(),
                vec![
                    Exercise::new(
                        "Exercise 1",
                        include_bytes!("../../assets/ui/exercise1/cover.png"),
                        String::from_utf8(
                            include_bytes!("../../assets/ui/exercise1/code.tm").to_vec(),
                        )
                        .unwrap(),
                    ),
                    Exercise::new(
                        "Exercise 2",
                        include_bytes!("../../assets/ui/exercise2/cover.png"),
                        String::from_utf8(
                            include_bytes!("../../assets/ui/exercise2/code.tm").to_vec(),
                        )
                        .unwrap(),
                    ),
                ],
            ),
            ("Chapter 2".to_string(), vec![]),
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

        egui::Window::new("Workbook") //TODO: t!("title.debug", self.lang))
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
                    });

                    ui.add(|ui: &mut egui::Ui| {
                        ui.set_min_height(350.0); // Set the minimum height to fill the available space
                        ui.separator()
                    });

                    ui.vertical_centered_justified(|ui| {
                        self.get_exercise(self.selected)
                            .image
                            .show_max_size(ui, Vec2::new(600.0, 500.0));

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

    fn get_exercise(&self, i: (usize, usize)) -> &Exercise {
        &self.exercises[i.0].1[i.1]
    }
}
