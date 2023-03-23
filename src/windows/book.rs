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
    exercises: Vec<Exercise>,
    selected: usize,
}

impl BookWindow {
    pub fn new(lang: &str) -> Self {
        let exercises = vec![
            Exercise::new(
                "Exercise 1",
                include_bytes!("../../assets/ui/exercise1/cover.png"),
                String::from_utf8(include_bytes!("../../assets/ui/exercise1/code.tm").to_vec())
                    .unwrap(),
            ),
            Exercise::new(
                "Exercise 2",
                include_bytes!("../../assets/ui/exercise2/cover.png"),
                String::from_utf8(include_bytes!("../../assets/ui/exercise2/code.tm").to_vec())
                    .unwrap(),
            ),
        ];

        Self {
            lang: String::from(lang),
            exercises,
            selected: 0,
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
                ui.horizontal(|ui|{
                    ui.vertical(|ui| {
                        ui.heading("Catalog"); //t!("title.exercises", self.lang));

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (i, exercise) in self.exercises.iter().enumerate() {
                                if ui
                                    .add_enabled(self.selected != i, egui::Button::new(&exercise.title))
                                    .clicked()
                                {
                                    self.selected = i;
                                }
                            }
                        });
                    });

                    ui.add(|ui: &mut egui::Ui| {
                        ui.set_min_height(350.0); // Set the minimum height to fill the available space
                        ui.separator()
                    });

                    ui.vertical_centered_justified(|ui| {
                        self.exercises[self.selected]
                            .image
                            .show_max_size(ui, Vec2::new(600.0, 500.0));

                        ui.horizontal(|ui| {
                            if ui
                                .add_enabled(self.selected > 0, egui::Button::new("Previous"))
                                .clicked()
                            {
                                self.selected -= 1;
                            }

                            ui.add_space(ui.available_width() - 50.0);

                            if ui
                                .add_enabled(
                                    self.selected < self.exercises.len() - 1,
                                    egui::Button::new("Next"),
                                )
                                .clicked()
                            {
                                self.selected += 1;
                            }
                        });

                        if ui.button("Use this exercise").clicked() {
                            code = Some(self.exercises[self.selected].code.clone());
                        }
                    });
                });
            });

        (active, code)
    }
}
