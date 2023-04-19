use eframe::egui;

use super::{exercise::Exercise, MAX_IMG_SIZE};


pub struct WorkbookEditorWindow {
    lang: String,
    exercises: Vec<(String, Vec<Exercise>)>,
    selected: (usize, usize),
}

impl WorkbookEditorWindow {
    pub fn new(lang: &str) -> Self {
        let exercises: Vec<(String, Vec<Exercise>)> = vec![];

        Self {
            lang: String::from(lang),
            exercises,
            selected: (0, 0),
        }
    }

    pub fn set_lang(&mut self, lang: &str) {
        self.lang = lang.to_string();
    }

    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        let mut active = true;

        egui::Window::new("Workbook") //TODO: t!("title.debug", self.lang))
            .id(egui::Id::new("exercises_window"))
            .resizable(true)
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

                            ui.separator();

                            if ui.button("Add Chapter").clicked() {
                                self.exercises.push((
                                    "New Chapter".to_string(),
                                    vec![Exercise::new(
                                        "New Exercise",
                                        None,
                                        String::from(include_str!("../../../assets/ui/exercise1/code.tm")),
                                    )],
                                ));
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
                            .show_max_size(ui, MAX_IMG_SIZE);

                        // Add expandable empty space
                        ui.allocate_space(egui::Vec2::new(0.0, (MAX_IMG_SIZE.y-self.get_exercise(self.selected)
                        .image.height() as f32)/3.5));


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
                    });
                });
            });

            active
        }
        
        fn get_exercise(&self, i: (usize, usize)) -> &Exercise {
            &self.exercises[i.0].1[i.1]
        }
    }