#![allow(dead_code)]

use crate::windows::workbook::error;
use eframe::{
    egui::{self, load::LoadError, Image},
    epaint::Vec2,
};
use serde::{
    self, de::Error, ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    fmt::{self, Debug},
    path::PathBuf,
};

use crate::console_err;

use super::MAX_IMG_SIZE;

#[derive(Clone)]
pub struct Cover<'a> {
    pub image: Image<'a>,
    raw_data: Vec<u8>,
    size: Vec2,
}

impl<'a> Serialize for Cover<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("cover", 2)?;
        s.serialize_field("data", &self.raw_data)?;
        s.serialize_field("size", &self.size)?;
        s.end()
    }
}

/// Temporary struct for deserialization
#[derive(Deserialize)]
struct CoverData {
    data: Vec<u8>,
    size: Vec2,
}

impl<'de, 'a> Deserialize<'de> for Cover<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Use the temporary struct for deserialization
        let CoverData { data, size } = CoverData::deserialize(deserializer)?;

        // Assuming you have a way to construct an Image from raw data and size,
        // or you can modify this part to fit your actual implementation.
        let image = match image::load_from_memory_with_format(&data, image::ImageFormat::Png) {
            Ok(i) => i,
            Err(e) => {
                error!("Error decoding image from the data in the file: {}", e);
                return Err(Error::custom(e));
            }
        };
        let image_buffer = image.to_rgba8();
        let pixels = Vec::from(image_buffer.as_flat_samples().as_slice());
        let uri = format!("bytes://decoded");

        Ok(Self {
            image: egui::Image::from_bytes(uri, pixels)
                .max_size(MAX_IMG_SIZE)
                .rounding(10.0),
            raw_data: data,
            size,
        })
    }
}

impl<'a> From<Image<'a>> for Cover<'a> {
    fn from(src: Image<'a>) -> Self {
        Self {
            image: src.clone().max_size(MAX_IMG_SIZE).rounding(10.0),
            raw_data: Vec::new(),
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
        let image_buffer = image.to_rgba8();
        let pixels = Vec::from(image_buffer.as_flat_samples().as_slice());

        let uri = format!("file://{}", f.to_str().unwrap());

        Ok(Self {
            image: egui::Image::from_uri(uri)
                .max_size(MAX_IMG_SIZE)
                .rounding(10.0),
            raw_data: pixels,
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
            cover.image = img.max_size(MAX_IMG_SIZE).rounding(10.0);
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
