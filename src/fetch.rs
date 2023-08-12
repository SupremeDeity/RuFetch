use sysinfo::System;

use crate::types::{Config, MemType, Time};
use ansi_term::{self, Color::*};
use std::iter::repeat;
use std::process::Command;
use std::str;
use sysinfo::{ComponentExt, DiskExt, ProcessorExt, SystemExt};

/// Prints the fetch results to the console.
///
/// The result depends on the config file or the fallback defaults.

pub fn print(conf: &Config, sys: &System) {
    if conf.show_hostname {
        print_hostname(&sys);
    }

    if conf.show_cpu {
        print_cpu(conf, &sys);
    }

    if conf.show_os {
        print_os(&sys);
    }

    if conf.show_kernel_version {
        print_kernel_ver(&sys);
    }

    if conf.show_de {
        print_desktop_environment();
    }

    if conf.show_uptime {
        print_uptime(conf, &sys);
    }

    if conf.show_disks {
        print_disks(&sys);
    }

    if conf.show_memory {
        print_mem(conf, &sys);
    }

    if conf.show_swap {
        print_swap(conf, &sys);
    }

    if conf.show_temperature {
        print_temps(&sys);
    }

    if conf.show_colors {
        print_colors(conf);
    }
}

fn print_hostname(sys: &System) {
    let host_name = sys.get_host_name();
    // Getting the user
    let user = get_user();

    if let Some(host_name) = &host_name {
        println!(
            "{}@{}",
            Blue.bold().paint(user),
            Blue.bold().paint(host_name)
        );
    }

    println!("{}", "-".repeat(30));

    if let Some(host_name) = &host_name {
        println!("{} {}", Blue.bold().paint("Host:"), *host_name);
    }
}

fn print_os(sys: &System) {
    let os = sys.get_name();

    if let Some(os) = &os {
        println!("{} {}", Blue.bold().paint("OS:"), os);
    }
}

fn print_uptime(conf: &Config, sys: &System) {
    match &conf.uptime_type {
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

fn print_kernel_ver(sys: &System) {
    let kernel_ver = sys.get_kernel_version();
    if let Some(kernel_ver) = &kernel_ver {
        println!("{} {}", Blue.bold().paint("Kernel Version:"), *kernel_ver);
    }
}

fn print_cpu(conf: &Config, sys: &System) {
    let cpu_str = format!(
        "{} {}",
        Blue.bold().paint("CPU:"),
        sys.get_global_processor_info().get_brand()
    );

    if *&conf.show_cores {
        println!("{} ({})", cpu_str, sys.get_processors().len());
    } else {
        println!("{}", cpu_str);
    }
}

fn print_disks(sys: &System) {
    for disk in sys.get_disks() {
        println!(
            "{}: {} ({:.2} GB / {:.2} GB)",
            Blue.bold().paint("Disk"),
            Yellow.bold().paint(disk.get_name().to_string_lossy()),
            (disk.get_total_space() - disk.get_available_space()) as f64
                / (1024.0 * 1024.0 * 1024.0),
            disk.get_total_space() as f64 / (1024 * 1024 * 1024) as f64
        )
    }
}

fn print_mem(conf: &Config, sys: &System) {
    match &conf.memory_type {
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

fn print_swap(conf: &Config, sys: &System) {
    sys.get_global_processor_info().get_brand();
    match &conf.memory_type {
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

fn print_colors(conf: &Config) {
    println!(
        "{}{}{}{}",
        Red.on(Red)
            .paint(format!("{:width$}", width = &conf.colors_width)),
        Green
            .on(Green)
            .paint(format!("{:width$}", width = &conf.colors_width)),
        Blue.on(Blue)
            .paint(format!("{:width$}", width = &conf.colors_width)),
        Yellow
            .on(Yellow)
            .paint(format!("{:width$}", width = &conf.colors_width))
    );
    println!(
        "{}{}{}{}",
        Black
            .on(Black)
            .paint(format!("{:width$}", width = &conf.colors_width)),
        White
            .on(White)
            .paint(format!("{:width$}", width = &conf.colors_width)),
        Purple
            .on(Purple)
            .paint(format!("{:width$}", width = &conf.colors_width)),
        Cyan.on(Cyan)
            .paint(format!("{:width$}", width = &conf.colors_width))
    );
}

fn print_temps(sys: &System) {
    println!();
    println!("{}", Red.bold().paint("Temperature"));
    println!(
        "{}",
        Red.bold().paint(repeat('-').take(20).collect::<String>())
    );

    for component in sys.get_components() {
        println!(
            "{}: {}°C",
            Blue.bold().paint(component.get_label()),
            component.get_temperature()
        );
    }
    println!();
}

fn get_user() -> String {
    let mut user_out = if cfg!(target_os = "windows") || cfg!(target_os = "linux") {
        // linux, windows
        Command::new("whoami").output().unwrap()
    } else {
        // darwin(mac)
        Command::new("id -un").output().expect("none")
    };
    let user: String = if (str::from_utf8(&user_out.stdout).unwrap()).ends_with("\n") {
        user_out.stdout.pop();
        str::from_utf8(&user_out.stdout).unwrap().to_string()
    } else {
        str::from_utf8(&user_out.stdout).unwrap().to_string()
    };
    user
}

/// --------------- Linux only --------------------
///
/// Gets the current desktop enviroment. The information might not be 100% accurate.
/// Skips if nothing useful is found.
fn print_desktop_environment() {
    if cfg!(target_os = "linux") {
        // linux
        match option_env!("DESKTOP_SESSION") {
            Some(val) => {
                let de_str = format!("{} {}", Blue.bold().paint("DE:"), val);

                println!("{}", de_str);
            }
            None => {}
        }
    } else {
        {}
    };
}
