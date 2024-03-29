use std::{fs::File, io::Write, str::FromStr};

use directories::ProjectDirs;

#[cfg(not(target_family = "wasm"))]
use log::{debug, error};

use serde::{Deserialize, Serialize};

use version::{version, Version};

use crate::{console_err, console_log, get_lang, Language};

const QUALIFIER: &str = "org";
const ORGANIZATION: &str = "margual56";
const APPLICATION: &str = "Turing Machine";

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub struct Config {
    version: Version,

    pub times_opened: u32,

    pub language: Language,

    /// Autosave is enabled by default. Has the user manually disabled it?
    autosave_disabled: bool,

    pub tape_size: f32,

    pub tape_speed: f32,

    pub threshold_inf_loop: usize,

    pub served_survey: bool,
}

impl Config {
    pub fn default() -> Self {
        Config {
            version: Version::from_str(version!()).unwrap(),
            times_opened: 0,
            language: get_lang(),
            autosave_disabled: false,
            tape_size: 100.0,
            tape_speed: 1.0,
            threshold_inf_loop: 100,
            served_survey: false,
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

                match toml::from_str::<Config>(
                    &(match std::fs::read_to_string(&file) {
                        Ok(s) => s,
                        Err(e) => {
                            console_err!("Cannot read configuration file: {}", e);
                            return None;
                        }
                    }),
                ) {
                    Ok(c) => {
                        let mut c = c;
                        c.increment_launches();
                        log::info!("Incremented launches: {}", c.times_opened);

                        if !Version::from_str(version!())
                            .unwrap()
                            .is_compatible_with(&c.version)
                        {
                            log::error!("A new version of the program is being used! Resetting configuration (it may be incompatible)...");

                            std::fs::remove_file(&file).unwrap();
                            let new_c = Config::default();
                            new_c.save();
                            return Some(new_c);
                        }

                        Some(c)
                    }
                    Err(e) => {
                        console_err!("Cannot parse configuration file: {}", e);
                        None
                    }
                }
            }
            None => {
                console_err!("Cannot find a valid directory to store the configuration file.");
                None
            }
        }
    }

    pub fn save(&self) {
        if let Some(dir) = ProjectDirs::from(
            QUALIFIER,    /*qualifier*/
            ORGANIZATION, /*organization*/
            APPLICATION,  /*application*/
        ) {
            if !dir.config_dir().try_exists().unwrap_or(true) {
                match std::fs::create_dir_all(dir.config_dir()) {
                    Ok(_) => {
                        console_log!("Created configuration directory: {:?}", dir.config_dir())
                    }
                    Err(e) => {
                        console_err!(
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
                    console_err!(
                        "Could not create configuration file {}: {}",
                        &file_path.to_string_lossy(),
                        e
                    );
                    return;
                }
            };

            log::info!(
                "Writing to configuration file: {}",
                &file_path.to_string_lossy()
            );

            let serialized_config = toml::to_string_pretty(self).unwrap();

            match file.write_all(serialized_config.as_bytes()) {
                Ok(_) => {}
                Err(e) => console_err!("Could not write configuration file: {}", e),
            };
        } else {
            console_err!("Could not open project directory");
        }
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn set_language(&mut self, l: Language) {
        self.language = l;
        self.save();
    }

    pub fn autosave_disabled(&self) -> bool {
        self.autosave_disabled
    }

    pub fn set_autosave_disabled(&mut self, b: bool) {
        self.autosave_disabled = b;
        self.save();
    }

    pub fn threshold_inf_loop(&self) -> usize {
        self.threshold_inf_loop
    }

    pub fn set_threshold_inf_loop(&mut self, t: usize) {
        self.threshold_inf_loop = t;
        self.save();
    }

    pub fn tape_size(&self) -> f32 {
        self.tape_size
    }

    pub fn set_tape_size(&mut self, s: f32) {
        self.tape_size = s;
        self.save();
    }

    pub fn tape_speed(&self) -> f32 {
        self.tape_speed
    }

    pub fn set_tape_speed(&mut self, s: f32) {
        self.tape_speed = s;
        self.save();
    }

    pub fn increment_launches(&mut self) {
        self.times_opened += 1;
        self.save();
    }

    pub fn survey_served(&mut self) {
        self.served_survey = true;
        self.save();
    }
}
