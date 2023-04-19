use egui_extras::RetainedImage;
use serde::{
    self,
    de::{Visitor, SeqAccess, Error as SerdeError},
    Deserialize, Deserializer, Serialize, Serializer, ser::SerializeStruct,
};
use serde_bytes::ByteBuf;
use std::fmt;

pub struct Exercise {
    pub image: RetainedImage,
    original_image: Vec<u8>,
    pub title: String,
    pub code: String,
}

impl Exercise {
    pub fn new(title: &str, img: Option<&[u8]>, code: String) -> Self {
        if let Some(img_bytes) = img {
            Self {
                image: RetainedImage::from_image_bytes(title, img_bytes).unwrap(),
                original_image: img_bytes.to_vec(),
                title: String::from(title),
                code: String::from(code),
            }
        }else{
            Self {
                image: RetainedImage::from_image_bytes(title, &[]).unwrap(),
                original_image: vec![],
                title: String::from(title),
                code: String::from(code),
            }
        }
        
    }
}

impl Serialize for Exercise {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Exercise", 4)?;

        // Ensure title is valid UTF-8
        let title = match String::from_utf8(self.title.clone().into_bytes()) {
            Ok(valid_title) => valid_title,
            Err(_) => {
                // Handle the case where the title is not valid UTF-8, e.g., by replacing invalid sequences with � (U+FFFD)
                String::from_utf8_lossy(&self.title.as_bytes()).to_string()
            }
        };

        state.serialize_field("title", &title)?;
        state.serialize_field("original_image", &self.original_image)?;
        state.serialize_field("code", &self.code)?;

        state.end()
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
                formatter.write_str("an Exercise in binary format")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Exercise, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let title: String = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::invalid_length(0, &self))?;
                let original_image: ByteBuf = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::invalid_length(1, &self))?;
                let code: String = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::invalid_length(2, &self))?;

                Ok(Exercise::new(&title, Some(&original_image.into_vec()), code))
            }
        }

        deserializer.deserialize_tuple(3, ExerciseVisitor)
    }
}

// Custom deserializer for the tuple (String, Vec<Exercise>)
fn deserialize_tuple<'de, D>(deserializer: D) -> Result<(String, Vec<Exercise>), D::Error>
where
    D: Deserializer<'de>,
{
    struct TupleVisitor;

    impl<'de> Visitor<'de> for TupleVisitor {
        type Value = (String, Vec<Exercise>);

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a tuple (String, Vec<Exercise>) in binary format")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<(String, Vec<Exercise>), A::Error>
        where
            A: SeqAccess<'de>,
        {
            let key: String = seq
                .next_element()?
                .ok_or_else(|| A::Error::invalid_length(0, &self))?;
            let value: Vec<Exercise> = seq
                .next_element()?
                .ok_or_else(|| A::Error::invalid_length(1, &self))?;

            Ok((key, value))
        }
    }

    deserializer.deserialize_tuple(2, TupleVisitor)
}