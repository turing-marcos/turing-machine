use eframe::egui;
use internationalization::t;
use log::{debug, error};
use serde::{self, Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, Write},
};

use super::{exercise::Exercise, MAX_IMG_SIZE};

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
                        Some(include_bytes!("../../../assets/ui/exercise1/cover.png")),
                        String::from(include_str!("../../../assets/ui/exercise1/code.tm")),
                    ),
                    Exercise::new(
                        "Exercise 2",
                        Some(include_bytes!("../../../assets/ui/exercise2/cover.png")),
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
                        if let Some(img) = &self.get_exercise(self.selected).image {
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
            let data = bincode::serialize(&self.exercises).unwrap();
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
            .add_filter("TuringMachine Workbook", &["wb"])
            .set_directory(&path)
            .pick_files();

        match file_path {
            Some(f) => {
                let file = File::open(&f[0]).expect("File not found");
                let reader = BufReader::new(file);

                match bincode::deserialize_from(reader) {
                    Ok(exercises) => {
                        debug!("Workbook loaded from {:?}", f[0]);
                        Some(exercises)
                    }
                    Err(e) => {
                        error!("Cannot load workbook: {}", e);
                        None
                    }
                }
            }
            None => {
                debug!("The path is not valid");
                None
            }
        }
    }
}
