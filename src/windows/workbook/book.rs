use std::{
    fmt,
    fs::File,
    io::{Read, Write},
};

use bincode::{deserialize, serialize};
use eframe::{egui, epaint::Vec2};
use egui_extras::RetainedImage;
use internationalization::t;
use log::{debug, error};
use serde::{
    self,
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize)]
pub struct Exercise {
    #[serde(skip_serializing)]
    image: RetainedImage,
    original_image: Vec<u8>,
    title: String,
    code: String,
}

impl Exercise {
    pub fn new(title: &str, img: &[u8], code: String) -> Self {
        Self {
            image: RetainedImage::from_image_bytes(title, img).unwrap(),
            original_image: img.to_vec(),
            title: String::from(title),
            code: String::from(code),
        }
    }
}

impl<'de> Deserialize<'de> for Exercise {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ExerciseVisitor;

        impl<'de> Visitor<'de> for ExerciseVisitor {
            type Value = Exercise;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Exercise")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Exercise, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut image_data: Option<Vec<u8>> = None;
                let mut title = None;
                let mut code = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "image_data" => {
                            image_data = Some(map.next_value()?);
                        }
                        "title" => {
                            title = Some(map.next_value()?);
                        }
                        "code" => {
                            code = Some(map.next_value()?);
                        }
                        _ => (),
                    }
                }

                let image_data =
                    image_data.ok_or_else(|| de::Error::missing_field("image_data"))?;
                let title = title.ok_or_else(|| de::Error::missing_field("title"))?;
                let code = code.ok_or_else(|| de::Error::missing_field("code"))?;

                let image = RetainedImage::from_image_bytes(&title, &image_data).unwrap();

                Ok(Exercise {
                    image,
                    original_image: image_data,
                    title,
                    code,
                })
            }
        }

        const FIELDS: &[&str] = &["image_data", "title", "code"];
        deserializer.deserialize_struct("Exercise", FIELDS, ExerciseVisitor)
    }
}

#[derive(Serialize, Deserialize)]
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
                        include_bytes!("../../../assets/ui/exercise1/cover.png"),
                        String::from(include_str!("../../../assets/ui/exercise1/code.tm")),
                    ),
                    Exercise::new(
                        "Exercise 2",
                        include_bytes!("../../../assets/ui/exercise2/cover.png"),
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

                        if ui.button("Save workbook").clicked() {
                            self.save_workbook();
                        }

                        if ui.button("Load workbook").clicked() {
                            let new_exercises = BookWindow::load_workbook();
                            if let Some(new_exercises) = new_exercises {
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

    pub fn save_workbook(&self) {
        let path = std::env::current_dir().unwrap();

        let file_path = rfd::FileDialog::new()
            .add_filter("Turing Machine Workbook", &["wb"])
            .set_directory(&path)
            .save_file();

        if let Some(f) = file_path {
            let data = serialize(&self.exercises).unwrap();
            let mut file = File::create(&f).unwrap();
            file.write_all(&data).unwrap();
            debug!("Workbook saved at {:?}", f);
        } else {
            error!("Cannot save workbook");
        }
    }

    pub fn load_workbook() -> Option<Vec<(String, Vec<Exercise>)>> {
        let path = std::env::current_dir().unwrap();

        let file_path = rfd::FileDialog::new()
            .add_filter("Turing Machine Workbook", &["wb"])
            .set_directory(&path)
            .save_file();

        if let Some(f) = file_path {
            let data = match File::open(&f) {
                Ok(mut file) => {
                    let mut data = Vec::new();
                    file.read_to_end(&mut data).unwrap();
                    data
                }
                Err(_) => {
                    error!("Cannot open workbook at {:?}", f);
                    return None;
                }
            };

            return deserialize(&data).unwrap();
        } else {
            error!("Cannot load workbook");
            return None;
        }
    }
}
