mod book;
mod exercise;
mod wb_editor;

pub use book::BookWindow as WorkbookWindow;
pub use wb_editor::WorkbookEditorWindow;

use std::io::BufReader;
use std::path::PathBuf;
use std::{fs::File, io::Write};

use eframe::egui;

use self::exercise::Exercise;
use eframe::epaint::ColorImage;
use log::{debug, error};

const MAX_IMG_SIZE: egui::Vec2 = egui::Vec2::new(600.0, 250.0);

fn pick_image() -> Option<PathBuf> {
    let path = std::env::current_dir().unwrap();

    let file_path = rfd::FileDialog::new()
        .add_filter("Image", &["png", "jpg", "jpeg"])
        .set_directory(&path)
        .pick_file();

    match file_path {
        Some(f) => Some(f),
        None => {
            debug!("The path is not valid");
            None
        }
    }
}

#[allow(dead_code)]
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
            debug!("The path is not valid");
            None
        }
    }
}

fn load_image() -> Option<ColorImage> {
    match pick_image() {
        Some(f) => {
            let image = match image::io::Reader::open(f) {
                Ok(img) => match img.decode() {
                    Ok(img_bytes) => img_bytes,
                    Err(e) => {
                        error!("Could not decode image: {:?}", e);
                        return None;
                    }
                },
                Err(e) => {
                    error!("Could not open image: {:?}", e);
                    return None;
                }
            };

            let size = [image.width() as _, image.height() as _];
            let image_buffer = image.to_rgba8();
            let pixels = image_buffer.as_flat_samples();
            Some(egui::ColorImage::from_rgba_unmultiplied(
                size,
                pixels.as_slice(),
            ))
        }
        None => {
            debug!("The path is not valid");
            None
        }
    }
}

fn raw_data_to_image(img_size: (u32, u32), data: &[u8]) -> ColorImage {
    let image = image::load_from_memory(data).expect("Failed to load image");

    // Check the image dimensions
    assert_eq!(image.width(), img_size.0);
    assert_eq!(image.height(), img_size.1);

    // Convert the image to raw RGB data
    let rgb_image = image.to_rgb8();
    let raw_data = rgb_image.into_raw();

    // Create the ColorImage
    ColorImage::from_rgb([img_size.0 as usize, img_size.1 as usize], &raw_data)
}

#[allow(dead_code)]
fn image_to_raw_data(color_image: &ColorImage) -> (usize, usize, Vec<u8>) {
    let size = color_image.size;
    let raw_data: Vec<u8> = color_image
        .pixels
        .iter()
        .flat_map(|p| p.to_array())
        .collect();
    (size[0], size[1], raw_data.to_vec())
}

pub fn save_workbook(exercises: &Vec<(String, Vec<Exercise>)>) {
    let path = std::env::current_dir().unwrap();

    let file_path = rfd::FileDialog::new()
        .add_filter("Turing Machine Workbook", &["wb"])
        .set_directory(&path)
        .save_file();

    if let Some(f) = file_path {
        let data = bincode::serialize(&exercises).unwrap();
        let mut file = File::create(&f).unwrap();
        file.write_all(&data).unwrap();
        debug!("Workbook saved at {:?}", f);
    } else {
        error!("Cannot save workbook");
    }
}

pub fn load_workbook() -> Option<Vec<(String, Vec<Exercise>)>> {
    let path = std::env::current_dir().unwrap();

    let file_path = rfd::FileDialog::new()
        .add_filter("TuringMachine Workbook", &["wb"])
        .set_directory(&path)
        .pick_files();

    match file_path {
        Some(f) => {
            let file = File::open(&f[0]).expect("File not found");
            let reader = BufReader::new(file);

            match bincode::deserialize_from(reader) {
                Ok(exercises) => {
                    debug!("Workbook loaded from {:?}", f[0]);
                    Some(exercises)
                }
                Err(e) => {
                    error!("Cannot load workbook: {}", e);
                    None
                }
            }
        }
        None => {
            debug!("The path is not valid");
            None
        }
    }
}
