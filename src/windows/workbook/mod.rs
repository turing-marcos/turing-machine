mod book;
mod exercise;
mod wb_editor;

pub use book::BookWindow as WorkbookWindow;
pub use wb_editor::WorkbookEditorWindow;

use eframe::egui;

use self::exercise::{Cover, Exercise};

type WorkbookChapter<'a> = (String, Vec<Exercise<'a>>);
type Workbook<'a> = Vec<WorkbookChapter<'a>>;

use crate::{console_err, console_log};

#[cfg(not(target_arch = "wasm32"))]
use {
    log::{debug, error},
    std::{
        fs::File,
        io::{Read, Write},
        path::PathBuf,
    },
};

use rfd;

const MAX_IMG_SIZE: egui::Vec2 = egui::Vec2::new(600.0, 250.0);

#[cfg(not(target_arch = "wasm32"))]
fn pick_image() -> Option<PathBuf> {
    let path = std::env::current_dir().unwrap();

    let file_path = rfd::FileDialog::new()
        .add_filter("Image", &["png", "jpg", "jpeg"])
        .set_directory(path)
        .pick_file();

    match file_path {
        Some(f) => Some(f),
        None => {
            console_log!("The path is not valid");
            None
        }
    }
}

#[allow(dead_code)]
#[cfg(not(target_arch = "wasm32"))]
fn load_image_bytes() -> Option<(u32, u32, Vec<u8>)> {
    match pick_image() {
        Some(f) => {
            let dynamic_image = image::open(f).expect("Failed to open image");
            let rgba_image = dynamic_image.to_rgba8();

            let width = rgba_image.width();
            let height = rgba_image.height();

            Some((width, height, rgba_image.into_raw()))
        }
        None => {
            console_log!("The path is not valid");
            None
        }
    }
}

fn load_image<'a>() -> Option<Cover<'a>> {
    #[cfg(target_arch = "wasm32")]
    {
        /*
        FIXME: Not working

        let window = web_sys::window().expect("Failed to get window");
        let document = window.document().expect("Failed to get document");
        let input = document
            .get_element_by_id("file-input")
            .expect("Failed to get element by id");
        let input = input.dyn_into::<web_sys::HtmlInputElement>().unwrap();

        let file = input.files().unwrap().get(0).unwrap();
        let reader = web_sys::FileReader::new().unwrap();

        reader.read_as_array_buffer(&file).unwrap();

        let closure = Closure::once_into_js(move |event: web_sys::Event| {
            let event = event.dyn_into::<web_sys::ProgressEvent>().unwrap();
            let array_buffer = event.target().unwrap().dyn_into::<web_sys::ArrayBuffer>().unwrap();
            let array = js_sys::Uint8Array::new(&array_buffer);
            let mut data = vec![0; array.length() as usize];
            array.copy_to(&mut data[..]);
            let image = image::load_from_memory(&data).expect("Failed to load image");

            // Check the image dimensions
            assert_eq!(image.width(), MAX_IMG_SIZE.x as u32);
            assert_eq!(image.height(), MAX_IMG_SIZE.y as u32);

            // Convert the image to raw RGB data
            let rgb_image = image.to_rgb8();
            let raw_data = rgb_image.into_raw();

            // Create the ColorImage
            ColorImage::from_rgb([MAX_IMG_SIZE.x as usize, MAX_IMG_SIZE.y as usize], &raw_data)
        });

        reader.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        Some(image)
        */

        None
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        match pick_image() {
            Some(f) => match Cover::new(f) {
                Ok(img) => Some(img),
                Err(e) => {
                    error!("Error opening the image: {}", e);
                    None
                }
            },
            None => {
                console_log!("The path is not valid");
                None
            }
        }
    }
}

pub fn save_workbook(exercises: &Workbook) {
    #[cfg(target_arch = "wasm32")]
    {
        console_err!("Whoops! I haven't implemented saving workbooks yet");
        /*
        FIXME: Not working

        let data = bincode::serialize(&exercises).unwrap();
        let data_url = format!(
            "data:application/octet-stream;base64,{}",
            base64.encode(&data)
        );
        web_sys::window().unwrap().open_with_url(&data_url).unwrap();
        */
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = std::env::current_dir().unwrap();
        let file_path = rfd::FileDialog::new()
            .add_filter("Turing Machine Workbook", &["wb"])
            .set_directory(path)
            .save_file();

        if let Some(mut f) = file_path {
            f.set_extension("wb");

            let data = bincode::serialize(&exercises).unwrap();
            let mut file = File::create(&f).unwrap();
            file.write_all(&data).unwrap();

            console_log!("Workbook saved at {:?}", f);

            drop(file);
        } else {
            console_err!("Cannot save workbook");
        }
    }
}

#[cfg(target_family = "wasm")]
pub async fn load_workbook<'a>() -> Option<Workbook<'a>> {
    let file_path = rfd::AsyncFileDialog::new()
        .add_filter("TuringMachine Workbook", &["wb"])
        .pick_file()
        .await;

    match file_path {
        Some(f) => {
            let reader: Vec<u8> = f.read().await;

            match bincode::deserialize::<Workbook<'a>>(&reader) {
                Ok(exercises) => {
                    console_log!("Workbook loaded from {:?}", &f);
                    Some(exercises)
                }
                Err(e) => {
                    console_err!("There was an error deserializing the workbook: {}", e);
                    None
                }
            }
        }
        None => {
            console_err!("There was an error opening the workbook file");
            None
        }
    }
}

#[cfg(not(target_family = "wasm"))]
pub fn load_workbook() -> Option<Workbook<'static>> {
    let path = std::env::current_dir().unwrap();

    let file_path = rfd::FileDialog::new()
        .add_filter("TuringMachine Workbook", &["wb"])
        .set_directory(path)
        .pick_file();

    match file_path {
        Some(f) => {
            let mut file = File::open(&f).expect("File not found");
            let mut reader: Vec<u8> = Vec::new();
            file.read_to_end(&mut reader).expect("Could not read file");

            console_log!("Read {} bytes", reader.len());

            match bincode::deserialize::<Workbook>(&reader) {
                Ok(exercises) => {
                    console_log!("Workbook loaded from {:?}", &f);
                    Some(exercises)
                }
                Err(e) => {
                    console_err!("Cannot load workbook: {}", e);
                    None
                }
            }
        }
        None => {
            console_log!("The path is not valid");
            None
        }
    }
}
