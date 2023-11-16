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
                    img.pixels.iter().flat_map(|p| p.to_array()).collect(),
                )),
                title: String::from(title),
                code,
            }
        } else {
            Self {
                image: None,
                original_image: None,
                title: String::from(title),
                code,
            }
        }
    }

    pub fn set_cover(&mut self, img: ColorImage) {
        self.image = Some(RetainedImage::from_color_image(&self.title, img.clone()));
        self.original_image = Some((
            img.width(),
            img.height(),
            img.pixels.iter().flat_map(|p| p.to_array()).collect(),
        ));
    }

    pub fn get_cover(&mut self) -> Option<&RetainedImage> {
        self.image.as_ref()
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

impl Clone for Exercise {
    fn clone(&self) -> Self {
        Self {
            image: match &self.image {
                Some(_) => match RetainedImage::from_image_bytes(
                    &self.title,
                    &self.original_image.clone().unwrap_or_default().2,
                ) {
                    Ok(img) => Some(img),
                    Err(_) => None,
                },
                None => None,
            },
            original_image: self.original_image.clone(),
            title: self.title.clone(),
            code: self.code.clone(),
        }
    }
}
