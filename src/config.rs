use serde::{Deserialize, Serialize};

use crate::thread_servo::AlternatingSettings;

use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PiperConfig {

    /// Location of music file to play
    pub music_file_location: String,

    /// Settings when door is moving back and fourth
    pub alternation_settings: AlternatingSettings,

    /// Door open fraction. Can be 0.0 or 1.0. Swap with `closed_fraction` to reverse.
    pub open_fraction: f32,

    /// Door closed fraction. Can be 0.0 or 1.0. Swap with `open_fraction`` to reverse.
    pub closed_fraction: f32,

    /// set to 0.0 to mean never. otherwise this is the number of minutes before auto-triggering
    pub idle_trigger_minutes: f32,

    /// set ot 0.5 to reduce the brightness of the LEDs to 50%
    pub brightness_factor:f32,
}

impl Default for PiperConfig {
    fn default() -> Self {
        PiperConfig{
            music_file_location:"./music/pied piper 3.mp3".to_owned(),
            alternation_settings: AlternatingSettings::default(),
            open_fraction: 1.0,
            closed_fraction: 0.0,
            idle_trigger_minutes: 30.0,
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
