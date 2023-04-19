use eframe::{egui, emath::Align};

use super::{exercise::Exercise, MAX_IMG_SIZE};

pub struct WorkbookEditorWindow {
    lang: String,
    chapters: Vec<(String, Vec<Exercise>)>,
    selected: (usize, usize),
}

impl WorkbookEditorWindow {
    pub fn new(lang: &str) -> Self {
        let exercises: Vec<(String, Vec<Exercise>)> = vec![];

        Self {
            lang: String::from(lang),
            chapters: exercises,
            selected: (0, 0),
        }
    }

    pub fn set_lang(&mut self, lang: &str) {
        self.lang = lang.to_string();
    }

    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        ctx.set_debug_on_hover(true);
        let mut active = true;

        egui::Window::new("Workbook editor") //TODO: t!("title.debug", self.lang))
            .id(egui::Id::new("editor_window"))
            .resizable(true)
            .open(&mut active)
            .auto_sized()
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.heading("Catalog"); //t!("title.exercises", self.lang));

                            egui::ScrollArea::vertical()
                                .id_source(egui::Id::new("scroll_list"))
                                .max_width(150.0)
                                .min_scrolled_height(ctx.available_rect().height() - 150.0)
                                .show(ui, |ui| {
                                    let mut old_selected = self.selected;

                                    for (section, (title, exercises)) in
                                        self.chapters.iter_mut().enumerate()
                                    {
                                        ui.collapsing(title.clone(), |ui| {
                                            for (i, exercise) in exercises.iter().enumerate() {
                                                if ui
                                                    .add_enabled(
                                                        self.selected.0 != section
                                                            || self.selected.1 != i,
                                                        egui::Button::new(&exercise.title),
                                                    )
                                                    .clicked()
                                                {
                                                    old_selected.0 = section;
                                                    old_selected.1 = i;
                                                }
                                            }

                                            ui.separator();

                                            if ui.button("Add Exercise").clicked() {
                                                exercises.push(WorkbookEditorWindow::new_exercise(
                                                    exercises.len(),
                                                ));
                                            }
                                        });
                                    }
                                    self.selected = old_selected;

                                    ui.separator();

                                    if ui.button("Add Chapter").clicked() {
                                        self.chapters.push(WorkbookEditorWindow::new_chapter(
                                            self.chapters.len(),
                                            0,
                                        ));
                                        self.selected = (self.chapters.len() - 1, 0);
                                    }
                                });
                        });

                        ui.separator();

                        ui.vertical_centered_justified(|ui| {
                            if let Some(ch) = self.chapters.get_mut(self.selected.0) {
                                ui.add(
                                    egui::TextEdit::singleline(&mut ch.0)
                                        .hint_text("Chapter title")
                                        .desired_width(0.0)
                                        .font(egui::TextStyle::Heading),
                                );

                                ui.separator();
                            }

                            if let Some(ex) = self.get_exercise(self.selected).as_mut() {
                                ui.add(
                                    egui::TextEdit::singleline(&mut ex.title)
                                        .hint_text("Exercise title")
                                        .desired_width(0.0)
                                        .font(egui::TextStyle::Heading),
                                );

                                if let Some(img) = &ex.image {
                                    img.show_max_size(ui, MAX_IMG_SIZE);
                                } else {
                                    // TODO: Draw a rectangle of size MAX_IMG_SIZE with a background color of 0.5
                                    let rect = egui::Rect::from_min_size(
                                        ui.cursor().left_top(),
                                        MAX_IMG_SIZE,
                                    );
                                    ui.painter().rect_filled(
                                        rect,
                                        10.0,
                                        egui::Color32::from_gray(24),
                                    );
                                    ui.allocate_exact_size(
                                        MAX_IMG_SIZE - egui::Vec2::new(0.0, 50.0),
                                        egui::Sense::focusable_noninteractive(),
                                    );
                                    ui.horizontal(|ui| {
                                        ui.add_space(15.0);
                                        if ui.button("Add image").clicked() {
                                            // TODO: Add image
                                        }
                                    });
                                }

                                ui.add_space(50.0);

                                ui.label("Exercise code:"); //t!("editor.code.header", self.lang));

                                let mut code = ex.code.clone();
                                egui::ScrollArea::vertical()
                                    .id_source(egui::Id::new("scroll_code"))
                                    .min_scrolled_height(300.0)
                                    .show(ui, |my_ui: &mut egui::Ui| {
                                        let editor = egui::TextEdit::multiline(&mut code)
                                            .code_editor()
                                            .desired_width(0.0);

                                        my_ui.add(editor);
                                    });

                                ex.code = code.clone();
                            }
                        });
                    });

                    ui.horizontal(|ui| {
                        if ui.button("Save workbook").clicked() {
                            println!("TODO: Save workbook");
                        }
                    });
                });
            });

        active
    }

    fn new_chapter(chapters_len: usize, exercises_len: usize) -> (String, Vec<Exercise>) {
        (
            format!("New Chapter {}", chapters_len + 1),
            vec![WorkbookEditorWindow::new_exercise(exercises_len)],
        )
    }

    fn new_exercise(exercises_len: usize) -> Exercise {
        Exercise::new(
            &format!("New Exercise {}", exercises_len + 1),
            None,
            String::new(),
        )
    }

    fn get_exercise(&mut self, i: (usize, usize)) -> Option<&mut Exercise> {
        if i.0 >= self.chapters.len() {
            return None;
        }
        if i.1 >= self.chapters[i.0].1.len() {
            return None;
        }

        Some(&mut self.chapters[i.0].1[i.1])
    }
}
