extern crate dirs;

use ansi_term::Color::*;
use serde::Deserialize;
use std::fs::File;
use std::path::Path;
use std::str;
use std::{io::Read, process::Command};
use sysinfo::{System, SystemExt};

#[derive(Deserialize)]
enum Time {
    Second,
    Minute,
    Hour,
}

impl Time {
    fn default() -> Self {
        Time::Hour
    }
}

#[derive(Deserialize)]
enum MemType {
    KB,
    MB,
    GB,
}

impl MemType {
    fn default() -> Self {
        MemType::GB
    }
}

// Need a better way for these two
fn default_bool() -> bool {
    true
}

fn default_usize() -> usize {
    3
}

#[derive(Deserialize)]
struct Config {
    #[serde(default = "default_bool")]
    show_os: bool,

    #[serde(default = "default_bool")]
    show_hostname: bool,

    #[serde(default = "default_bool")]
    show_uptime: bool,

    #[serde(default = "default_bool")]
    show_kernel_version: bool,

    #[serde(default = "default_bool")]
    show_memory: bool,

    #[serde(default = "default_bool")]
    show_swap: bool,

    #[serde(default = "default_bool")]
    show_colors: bool,

    #[serde(default = "default_usize")]
    colors_width: usize,

    #[serde(default = "Time::default")]
    uptime_type: Time,

    #[serde(default = "MemType::default")]
    memory_type: MemType,
}

fn main() {
    // Get the system info
    let sys = System::new_all();
    //let enabled = ansi_term::enable_ansi_support(); // for some reason this isnt available.

    // Getting the user
    let mut user_out = if cfg!(target_os = "windows") || cfg!(target_os = "linux") {
        // linux, windows
        Command::new("whoami").output().expect("none")
    } else {
        // darwin(mac)
        Command::new("id -un").output().expect("none")
    };

    let user = if (str::from_utf8(&user_out.stdout).unwrap()).ends_with("\n") {
        user_out.stdout.pop();
        str::from_utf8(&user_out.stdout).unwrap()
    } else {
        str::from_utf8(&user_out.stdout).unwrap()
    };

    let default_config = r#"
        show_os = true
        show_hostname = true
        show_uptime = true
        show_kernel_version = true
        show_memory = true
        show_swap = true
        show_colors = true
        colors_width = 3
        uptime_type = "Minute"
        memory_type = "GB"
    "#;
    let config_path = std::format!(
        "{}/ru_fetch/config.toml",
        dirs::config_dir().unwrap().as_path().to_str().unwrap()
    );

    let config: Config = if cfg!(target_os = "linux") && Path::new(&config_path).exists() {
        let f = File::open(&config_path);
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

    let host_name = sys.get_host_name();

    if let Some(host_name) = &host_name {
        println!(
            "{}@{}",
            Blue.bold().paint(user),
            Blue.bold().paint(host_name)
        );
    }
    println!("{:-<1$}", "", 30);

    if config.show_os {
        let os = sys.get_name();
        if let Some(os) = &os {
            println!("{} {}", Blue.bold().paint("OS:"), os);
        }
    }

    if config.show_hostname {
        if let Some(host_name) = &host_name {
            println!("{} {}", Blue.bold().paint("Host:"), *host_name);
        }
    }

    if config.show_uptime {
        match config.uptime_type {
            Time::Second => {
                let uptime_sec = sys.get_uptime();
                println!("{} {:.2} sec(s)", Blue.bold().paint("Uptime: "), uptime_sec);
            }
            Time::Minute => {
                let uptime_min: f64 = sys.get_uptime() as f64 / 60 as f64;
                println!("{} {:.2} min(s)", Blue.bold().paint("Uptime:"), uptime_min);
            }
            Time::Hour => {
                let uptime_hour: f64 = sys.get_uptime() as f64 / 3600 as f64;
                println!(
                    "{} {:.2} hour(s)",
                    Blue.bold().paint("Uptime: "),
                    uptime_hour
                );
            }
        }
    }

    if config.show_kernel_version {
        let kernel_ver = sys.get_kernel_version();
        if let Some(kernel_ver) = &kernel_ver {
            println!("{} {}", Blue.bold().paint("Kernel Version:"), *kernel_ver);
        }
    }
    if config.show_memory {
        match config.memory_type {
            MemType::KB => println!(
                "{} {:.2} KB / {:.2} KB",
                Blue.bold().paint("Memory:"),
                sys.get_used_memory(),
                sys.get_total_memory()
            ),
            MemType::MB => {
                println!(
                    "{} {:.2} MB / {:.2} MB",
                    Blue.bold().paint("Memory:"),
                    sys.get_used_memory() as f64 / 1e+3,
                    sys.get_total_memory() as f64 / 1e+3
                )
            }
            MemType::GB => {
                println!(
                    "{} {:.2} GB / {:.2} GB",
                    Blue.bold().paint("Memory:"),
                    sys.get_used_memory() as f64 / 1e+6,
                    sys.get_total_memory() as f64 / 1e+6
                )
            }
        }
    }

    if config.show_swap {
        match config.memory_type {
            MemType::KB => println!(
                "{} {:.2} KB / {:.2} KB",
                Blue.bold().paint("Swap:"),
                sys.get_used_swap(),
                sys.get_total_swap()
            ),
            MemType::MB => {
                println!(
                    "{} {:.2} MB / {:.2} MB",
                    Blue.bold().paint("Swap:"),
                    sys.get_used_swap() as f64 / 1e+3,
                    sys.get_total_swap() as f64 / 1e+3
                )
            }
            MemType::GB => {
                println!(
                    "{} {:.2} GB / {:.2} GB",
                    Blue.bold().paint("Swap:"),
                    sys.get_used_swap() as f64 / 1e+6,
                    sys.get_total_swap() as f64 / 1e+6
                )
            }
        }
    }

    if config.show_colors {
        println!(
            "{}{}{}{}",
            Red.on(Red)
                .paint(format!("{:width$}", width = config.colors_width)),
            Green
                .on(Green)
                .paint(format!("{:width$}", width = config.colors_width)),
            Blue.on(Blue)
                .paint(format!("{:width$}", width = config.colors_width)),
            Yellow
                .on(Yellow)
                .paint(format!("{:width$}", width = config.colors_width))
        );
        println!(
            "{}{}{}{}",
            Black
                .on(Black)
                .paint(format!("{:width$}", width = config.colors_width)),
            White
                .on(White)
                .paint(format!("{:width$}", width = config.colors_width)),
            Purple
                .on(Purple)
                .paint(format!("{:width$}", width = config.colors_width)),
            Cyan.on(Cyan)
                .paint(format!("{:width$}", width = config.colors_width))
        )
    }
}
