use std::{fs::File, io::Read, path::Path};

use crate::types::{Config, MemType, Time};
use ansi_term::{self, Color::*};
pub use serde::Deserialize;

impl Time {
    pub fn default() -> Self {
        Time::Hour
    }
}

impl MemType {
    pub fn default() -> Self {
        MemType::GB
    }
}

impl Config {
    /// Fetches config and returns a new [Config] instance.
    ///
    /// # Panic
    /// This code should not panic under normal circumstances.

    pub fn new() -> Config {
        let default_config = r#"
        show_os = true
        show_hostname = true
        show_uptime = true
        show_kernel_version = true
        show_memory = true
        show_swap = true
        show_de = true
        show_temperature = true
        show_colors = true
        show_cpu = true
        show_cores = true
        show_disks = true
        colors_width = 3
        uptime_type = "Minute"
        memory_type = "GB"
    "#;

        let config_path = std::format!(
            "{}/ru_fetch/config.toml",
            dirs::config_dir().unwrap().as_path().to_str().unwrap()
        );

        let config: Config = if cfg!(target_os = "linux") && Path::new(&config_path).exists() {
            let f = File::open(&config_path); // no unwrap() since other errors can occur too.
            match f {
                Ok(mut file) => {
                    let mut contents = String::new();
                    match file.read_to_string(&mut contents) {
                        Ok(_) => {}
                        Err(_) => toml::from_str(default_config).unwrap(),
                    }

                    match toml::from_str(&contents) {
                        Ok(config) => config,
                        Err(error) => {
                            println!(
                                "{}",
                                Red.bold()
                                    .paint("Error in config, falling back to default config.")
                            );
                            println!(
                                "{} {}",
                                Red.bold().paint(error.to_string()),
                                Blue.bold().paint("(line, column may differ from actual)")
                            );
                            toml::from_str(default_config).unwrap()
                        }
                    }
                }
                Err(_) => toml::from_str(default_config).unwrap(),
            }
        } else {
            toml::from_str(default_config).unwrap()
        };

        config
    }

    /// Returns the default value for bool fields of [Config]
    pub fn default_bool() -> bool {
        true
    }

    /// Returns the default value for usize fields of [Config]
    pub fn default_usize() -> usize {
        3
    }
}
