use eframe::epaint::ColorImage;
use egui_extras::RetainedImage;
use serde::{self, Deserialize, Serialize};
use std::fmt::{self, Debug};

#[derive(Serialize, Deserialize)]
pub struct Exercise {
    #[serde(skip)]
    pub image: Option<RetainedImage>,
    original_image: Option<(usize, usize, Vec<u8>)>,
    pub title: String,
    pub code: String,
}

impl Exercise {
    pub fn new(title: &str, image: Option<ColorImage>, code: String) -> Self {
        if let Some(img) = image {
            Self {
                image: Some(RetainedImage::from_color_image(title, img.clone())),
                original_image: Some((
                    img.width(),
                    img.height(),
                    img.pixels.iter().map(|p| p.to_array()).flatten().collect(),
                )),
                title: String::from(title),
                code: String::from(code),
            }
        } else {
            Self {
                image: None,
                original_image: None,
                title: String::from(title),
                code: String::from(code),
            }
        }
    }

    pub fn set_cover(&mut self, img: ColorImage) {
        self.image = Some(RetainedImage::from_color_image(&self.title, img.clone()));
        self.original_image = Some((
            img.width(),
            img.height(),
            img.pixels.iter().map(|p| p.to_array()).flatten().collect(),
        ));
    }

    pub fn get_cover(&mut self) -> Option<&RetainedImage> {
        match self.image {
            Some(ref img) => Some(img),
            None => {
                if let Some(img) = &self.original_image {
                    self.set_cover(ColorImage::from_rgba_premultiplied([img.0, img.1], &img.2));
                    self.image.as_ref()
                } else {
                    None
                }
            }
        }
    }
}

impl Debug for Exercise {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Exercise")
            .field("title", &self.title)
            .field("code", &self.code)
            .finish()
    }
}
