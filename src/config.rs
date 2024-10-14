use serde::{Deserialize, Serialize};

use crate::thread_servo::AlternatingSettings;

use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PiperConfig {
    pub music_file_location: String,
    pub alternation_settings: AlternatingSettings,
    pub open_fraction: f32,
    pub closed_fraction: f32,
    pub idle_trigger_minutes: f32, // in minutes
    pub brightness_factor:f32,
}

impl Default for PiperConfig {
    fn default() -> Self {
        PiperConfig{
            music_file_location:"./music/pied piper 3.mp3".to_owned(),
            alternation_settings: AlternatingSettings::default(),
            open_fraction: 1.0,
            closed_fraction: 0.0,
            idle_trigger_minutes: 10.0,
            brightness_factor:1.0,
        }
    }
}

impl PiperConfig {
    pub fn load_from_file(path: &str) -> Self {
        let path = Path::new(path);

        // Resolve the full path
        let full_path = match fs::canonicalize(&path) {
            Ok(p) => p,
            Err(_) => path.to_path_buf(),
        };

        println!("Loading configuration from: {}", full_path.display());

        let config: PiperConfig = match fs::read_to_string(&path) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(cfg) => cfg,
                Err(err) => {
                    eprintln!(
                        "Failed to deserialize config file: {}. Using default configuration.",
                        err
                    );
                    PiperConfig::default()
                }
            },
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    // File does not exist, create it with default values
                    let default_config = PiperConfig::default();
                    let serialized = match serde_json::to_string_pretty(&default_config) {
                        Ok(s) => s,
                        Err(err) => {
                            eprintln!(
                                "Failed to serialize default configuration: {}. Using default configuration.",
                                err
                            );
                            return default_config;
                        }
                    };

                    if let Err(err) = fs::write(&path, serialized) {
                        eprintln!(
                            "Failed to create default config file: {}. Using default configuration.",
                            err
                        );
                    } else {
                        println!("Created default config file at: {}", full_path.display());
                    }
                    default_config
                } else {
                    eprintln!(
                        "Failed to read config file: {}. Using default configuration.",
                        err
                    );
                    PiperConfig::default()
                }
            }
        };

        config
    }
}
