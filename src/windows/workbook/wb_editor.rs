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

        egui::Window::new("Workbook editor") //TODO: t!("title.debug", self.lang))
            .id(egui::Id::new("editor_window"))
            .resizable(true)
            .open(&mut active)
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("Catalog"); //t!("title.exercises", self.lang));

                        egui::ScrollArea::vertical().show(ui, |ui| {
                            for (section, (title, exercises)) in self.exercises.iter_mut().enumerate() {
                                ui.collapsing(title.clone(), |ui| {
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

                                    if ui.button("Add Exercise").clicked() {
                                        exercises.push(Exercise::new(
                                            "New Exercise",
                                            None,
                                            String::new(),
                                        ));
                                    }
                                });
                            }

                            ui.separator();

                            if ui.button("Add Chapter").clicked() {
                                self.selected = (self.exercises.len(), 0);

                                self.exercises.push((
                                    "New Chapter".to_string(),
                                    vec![Exercise::new(
                                        "New Exercise",
                                        None,
                                        String::new(),
                                    )],
                                ));
                            }
                        });
                    });

                    ui.vertical_centered_justified(|ui| {
                        if let Some(ex) = self.get_exercise(self.selected).as_mut()  {
                            ex.image
                            .show_max_size(ui, MAX_IMG_SIZE);

                            ui.separator();
                            
                            let code = ex.code.clone();
                            ui.code(&code);

                            ex.code = code.clone();
                        }
                    });
                });
            });

            active
        }
        
        fn get_exercise(&mut self, i: (usize, usize)) -> Option<&mut Exercise> {
            if i.0 >= self.exercises.len() {
                return None;
            }
            if i.1 >= self.exercises[i.0].1.len() {
                return None;
            }

            Some(&mut self.exercises[i.0].1[i.1])
        }
    }
