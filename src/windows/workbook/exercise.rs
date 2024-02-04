#![allow(dead_code)]

use crate::windows::workbook::error;
use eframe::{
    egui::{self, load::LoadError, Image},
    epaint::Vec2,
};
use image::DynamicImage;
use serde::{
    self, de::Error, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
};
use std::io::Cursor;
use std::{
    fmt::{self, Debug},
    path::PathBuf,
};

use crate::console_err;

use super::MAX_IMG_SIZE;

const IMG_ROUNDING: f32 = 10.0;

#[derive(Clone, Debug)]
pub struct Cover<'a> {
    pub image: Image<'a>,
    orig_image: DynamicImage,
    size: Vec2,
}

impl<'a> Serialize for Cover<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("cover", 3)?;

        let mut img_bytes: Vec<u8> = Vec::new();
        self.orig_image
            .write_to(
                &mut Cursor::new(&mut img_bytes),
                image::ImageOutputFormat::Png,
            )
            .unwrap();

        s.serialize_field(
            "uri",
            &String::from(self.image.source().uri().unwrap_or("bytes://unknown.png")),
        )?;
        s.serialize_field("data", &img_bytes)?;
        s.serialize_field("size", &self.size)?;
        s.end()
    }
}

/// Temporary struct for deserialization
#[derive(Deserialize)]
struct CoverData {
    uri: String,
    data: Vec<u8>,
    size: Vec2,
}

impl<'de, 'a> Deserialize<'de> for Cover<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Use the temporary struct for deserialization
        let CoverData { uri, data, size } = CoverData::deserialize(deserializer)?;

        let image: DynamicImage =
            image::load_from_memory_with_format(&data, image::ImageFormat::Png)
                .map_err(D::Error::custom)?;

        Ok(Self {
            image: egui::Image::from_bytes(uri, data)
                .max_size(MAX_IMG_SIZE)
                .rounding(IMG_ROUNDING),
            orig_image: image,
            size,
        })
    }
}

impl<'a> From<Image<'a>> for Cover<'a> {
    fn from(src: Image<'a>) -> Self {
        let image = src.clone().max_size(MAX_IMG_SIZE).rounding(IMG_ROUNDING);

        let bytes = match src.source() {
            egui::ImageSource::Bytes { uri: _, bytes } => bytes,
            _ => {
                console_err!("Could not get bytes from image source");
                return Self {
                    image,
                    orig_image: DynamicImage::new_rgba8(1, 1),
                    size: src.size().unwrap_or_default(),
                };
            }
        };
        Self {
            image,
            orig_image: image::load_from_memory_with_format(&bytes, image::ImageFormat::Png)
                .unwrap_or_default(),
            size: src.size().unwrap_or_default(),
        }
    }
}

impl<'a> Cover<'a> {
    pub fn new(f: PathBuf) -> Result<Self, LoadError> {
        let image = match image::io::Reader::open(&f) {
            Ok(img) => match img.decode() {
                Ok(img_bytes) => img_bytes,
                Err(e) => {
                    console_err!("Could not decode image: {:?}", e);
                    return Err(LoadError::NotSupported);
                }
            },
            Err(e) => {
                console_err!("Could not open image: {:?}", e);
                return Err(LoadError::NotSupported);
            }
        };

        let size = Vec2::new(image.width() as _, image.height() as _);

        let uri = format!("file://{}", f.to_str().unwrap());

        Ok(Self {
            image: egui::Image::from_uri(uri)
                .max_size(MAX_IMG_SIZE)
                .rounding(IMG_ROUNDING),
            orig_image: image,
            size,
        })
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Exercise<'a> {
    image: Option<Cover<'a>>,
    pub title: String,
    pub code: String,
}

impl<'a> Exercise<'a> {
    pub fn new(title: &str, image: Option<Cover<'a>>, code: String) -> Self {
        Self {
            image,
            title: String::from(title),
            code,
        }
    }

    pub fn set_cover(&mut self, cov: Cover<'a>) {
        self.image = Some(cov);
    }

    pub fn set_cover_img(&mut self, img: Image<'a>) {
        if let Some(cover) = self.image.as_mut() {
            cover.image = img.max_size(MAX_IMG_SIZE).rounding(IMG_ROUNDING);
        } else {
            self.image = Some(img.into())
        }
    }

    pub fn get_cover(&self) -> Option<&Image<'a>> {
        if let Some(cover) = &self.image {
            Some(&cover.image)
        } else {
            None
        }
    }
}

impl<'a> Debug for Exercise<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Exercise")
            .field("title", &self.title)
            .field("code", &self.code)
            .finish()
    }
}
