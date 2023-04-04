use egui_extras::RetainedImage;
use serde::{
    self,
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::fmt;

#[derive(Serialize)]
pub struct Exercise {
    #[serde(skip_serializing)]
    pub image: RetainedImage,
    original_image: Vec<u8>,
    pub title: String,
    pub code: String,
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