use std::{fs::File, io::Write};

use directories::ProjectDirs;
use log::{error, info};

use serde::{Deserialize, Serialize};
use toml;
use version::Version;

use crate::{get_lang, Language};

const QUALIFIER: &str = "org";
const ORGANIZATION: &str = "margual56";
const APPLICATION: &str = "Turing Machine";

#[derive(Serialize, Deserialize)]
pub struct Config {
    version: Version,

    times_opened: u32,

    language: Language,

    /// Autosave is enabled by default. Has the user manually disabled it?
    autosave_disabled: bool,

    tape_size: f32,

    tape_speed: f32,

    threshold_inf_loop: usize,
}

impl Config {
    pub fn default() -> Self {
        Config {
            version: Version::get().unwrap(),
            times_opened: 0,
            language: get_lang(),
            autosave_disabled: false,
            tape_size: 100.0,
            tape_speed: 1.0,
            threshold_inf_loop: 100,
        }
    }

    pub fn load() -> Option<Self> {
        match ProjectDirs::from(
            QUALIFIER,    /*qualifier*/
            ORGANIZATION, /*organization*/
            APPLICATION,  /*application*/
        ) {
            Some(dir) => {
                let file = dir.config_dir().join("config.toml");

                log::info!("Loading configuration file: {:?}", file);

                match toml::from_str(
                    &(match std::fs::read_to_string(file) {
                        Ok(s) => s,
                        Err(e) => {
                            error!("Cannot read configuration file: {}", e);
                            return None;
                        }
                    }),
                ) {
                    Ok(c) => Some(c),
                    Err(e) => {
                        error!("Cannot parse configuration file: {}", e);
                        None
                    }
                }
            }
            None => {
                error!("Cannot find a valid directory to store the configuration file.");
                return None;
            }
        }
    }

    pub fn save(&self) {
        match ProjectDirs::from(
            QUALIFIER,    /*qualifier*/
            ORGANIZATION, /*organization*/
            APPLICATION,  /*application*/
        ) {
            Some(dir) => {
                if !dir.config_dir().try_exists().unwrap_or(true) {
                    match std::fs::create_dir_all(dir.config_dir()) {
                        Ok(_) => {
                            info!("Created configuration directory: {:?}", dir.config_dir())
                        }
                        Err(e) => {
                            error!(
                                "Could not create configuration directory {:?}: {}",
                                dir.config_dir(),
                                e
                            );
                            return;
                        }
                    };
                }

                let file_path = dir.config_dir().join("config.toml");

                let mut file: File = match File::create(&file_path) {
                    Ok(f) => f,
                    Err(e) => {
                        error!(
                            "Could not create configuration file {}: {}",
                            &file_path.to_string_lossy(),
                            e
                        );
                        return;
                    }
                };

                log::info!(
                    "Creating configuration file: {}",
                    &file_path.to_string_lossy()
                );

                let serialized_config = toml::to_string_pretty(self).unwrap();

                match file.write_all(serialized_config.as_bytes()) {
                    Ok(_) => {}
                    Err(e) => error!("Could not write configuration file: {}", e),
                };
            }
            None => {}
        };
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn autosave_disabled(&self) -> bool {
        self.autosave_disabled
    }

    pub fn threshold_inf_loop(&self) -> usize {
        self.threshold_inf_loop
    }

    pub fn tape_size(&self) -> f32 {
        self.tape_size
    }

    pub fn tape_speed(&self) -> f32 {
        self.tape_speed
    }
}
