use eframe::egui;
use internationalization::t;

use super::{
    exercise::Exercise, load_image, save_workbook, Workbook, WorkbookChapter, MAX_IMG_SIZE,
};

pub struct WorkbookEditorWindow<'a> {
    lang: String,
    chapters: Workbook<'a>,
    selected: (usize, usize),
}

impl<'a> WorkbookEditorWindow<'a> {
    pub fn new(lang: &str) -> Self {
        let exercises: Workbook = vec![];

        Self {
            lang: String::from(lang),
            chapters: exercises,
            selected: (0, 0),
        }
    }

    pub fn set_lang(&mut self, lang: &str) {
        self.lang = String::from(lang);
    }

    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        let mut active = true;

        egui::Window::new(t!("title.workbook.editor", &self.lang))
            .id(egui::Id::new("editor_window"))
            .resizable(true)
            .open(&mut active)
            .auto_sized()
            .show(ctx, |ui| {
                let lang = self.lang.clone();
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            let lang = self.lang.clone();
                            ui.heading(t!("heading.workbook.catalog", &lang));

                            egui::ScrollArea::vertical()
                                .id_source(egui::Id::new("scroll_list"))
                                .max_width(150.0)
                                .min_scrolled_height(ctx.available_rect().height() - 150.0)
                                .show(ui, |ui| {
                                    let lang = self.lang.clone();
                                    let mut old_selected = self.selected;

                                    for (section, (title, exercises)) in
                                        self.chapters.iter_mut().enumerate()
                                    {
                                        ui.collapsing(title.clone(), |ui| {
                                            let lang = self.lang.clone();
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

                                            if ui
                                                .button(t!("btn.editor.add_exercise", &lang))
                                                .clicked()
                                            {
                                                exercises.push(WorkbookEditorWindow::new_exercise(
                                                    exercises.len(),
                                                    "en",
                                                ));
                                            }
                                        });
                                    }
                                    self.selected = old_selected;

                                    ui.separator();

                                    if ui.button(t!("btn.editor.add_chapter", lang)).clicked() {
                                        self.chapters.push(WorkbookEditorWindow::new_chapter(
                                            self.chapters.len(),
                                            0,
                                            "en",
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
                                        .hint_text(t!("tooltip.editor.chapter_title", lang))
                                        .desired_width(0.0)
                                        .font(egui::TextStyle::Heading),
                                );

                                ui.separator();
                            }

                            if let Some(ex) = self.get_exercise(self.selected) {
                                ui.add(
                                    egui::TextEdit::singleline(
                                        &mut self.chapters[self.selected.0].1[self.selected.1]
                                            .title,
                                    )
                                    .hint_text(t!("tooltip.editor.chapter_title", lang))
                                    .desired_width(0.0)
                                    .font(egui::TextStyle::Heading),
                                );

                                if let Some(img) = ex.get_cover() {
                                    ui.add_sized(MAX_IMG_SIZE, img.clone().shrink_to_fit());
                                } else {
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
                                        if ui.button(t!("btn.editor.add_image", lang)).clicked() {
                                            if let Some(img) = load_image() {
                                                self.chapters[self.selected.0].1[self.selected.1]
                                                    .set_cover(img);
                                            }
                                        }
                                    });
                                }

                                ui.add_space(50.0);

                                ui.label(t!("lbl.editor.exercise_code", lang) + ":"); //t!("editor.code.header", self.lang));

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

                                self.chapters[self.selected.0].1[self.selected.1].code =
                                    code.clone();
                            }
                        });
                    });

                    ui.horizontal(|ui| {
                        if ui.button(t!("btn.editor.save_workbook", lang)).clicked() {
                            save_workbook(&self.chapters);
                        }
                    });
                });
            });

        active
    }

    fn new_chapter(chapters_len: usize, exercises_len: usize, lang: &str) -> WorkbookChapter {
        (
            t!(
                "lbl.editor.new_chapter",
                num: (chapters_len + 1).to_string().as_str(),
                lang
            ),
            vec![WorkbookEditorWindow::new_exercise(exercises_len, lang)],
        )
    }

    fn new_exercise(exercises_len: usize, lang: &str) -> Exercise {
        Exercise::new(
            &t!(
                "lbl.editor.new_exercise",
                num: (exercises_len + 1).to_string().as_str(),
                lang
            ),
            None,
            String::new(),
        )
    }

    /// Returns *a copy* of an exercise.
    fn get_exercise(&self, i: (usize, usize)) -> Option<Exercise<'a>> {
        if i.0 >= self.chapters.len() {
            return None;
        }
        if i.1 >= self.chapters[i.0].1.len() {
            return None;
        }

        Some(self.chapters[i.0].1[i.1].clone())
    }
}
