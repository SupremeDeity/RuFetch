use sysinfo::{Components, Disks, System, User, Users};

use crate::types::{Config, MemType, Time};
use ansi_term::{self, Color::*};
use std::iter::repeat;

pub fn print(conf: &Config, sys: &mut System) {
    sys.refresh_all();

    if conf.show_hostname {
        print_hostname();
    }

    if conf.show_cpu {
        print_cpu(&conf.show_cores, &sys);
    }

    if conf.show_os {
        print_os();
    }

    if conf.show_kernel_version {
        print_kernel_ver();
    }

    if conf.show_de {
        print_desktop_environment();
    }

    if conf.show_uptime {
        print_uptime(&conf.uptime_type);
    }

    if conf.show_disks {
        print_disks();
    }

    if conf.show_memory {
        print_mem(&conf.memory_type, &sys);
    }

    if conf.show_swap {
        print_swap(&conf.memory_type, &sys);
    }

    if conf.show_temperature {
        print_temps();
    }

    if conf.show_colors {
        print_colors(&conf.colors_width);
    }
}

fn print_hostname() {
    let host_name = System::host_name();
    let user = whoami::username();
    if let Some(host_name) = &host_name {
        println!(
            "{}@{}",
            Blue.bold().paint(user),
            Blue.bold().paint(host_name)
        );
    }

    println!("{}", "-".repeat(30));

    if let Some(host_name) = &host_name {
        println!("{} {}", Blue.bold().paint("Host:"), host_name);
    }
}

fn print_os() {
    println!("{} {}", Blue.bold().paint("OS:"), whoami::distro());
}

fn print_uptime(uptime_type: &Time) {
    let uptime = System::uptime();
    match uptime_type {
        Time::Second => {
            println!("{} {:.2} sec(s)", Blue.bold().paint("Uptime: "), uptime);
        }
        Time::Minute => {
            let uptime_min = uptime as f64 / 60.0;
            println!("{} {:.2} min(s)", Blue.bold().paint("Uptime:"), uptime_min);
        }
        Time::Hour => {
            let uptime_hour = uptime as f64 / 3600.0;
            println!(
                "{} {:.2} hour(s)",
                Blue.bold().paint("Uptime: "),
                uptime_hour
            );
        }
    }
}

fn print_kernel_ver() {
    if let Some(kernel_ver) = System::kernel_version() {
        println!("{} {}", Blue.bold().paint("Kernel Version:"), kernel_ver);
    }
}

fn print_cpu(show_cores: &bool, sys: &System) {
    let cpu_str = format!("{} {}", Blue.bold().paint("CPU:"), sys.cpus()[0].brand());

    if *show_cores {
        println!("{} ({})", cpu_str, sys.cpus().len());
    } else {
        println!("{}", cpu_str);
    }
}

fn print_disks() {
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!(
            "{}: {} ({:.2} GB / {:.2} GB)",
            Blue.bold().paint("Disk"),
            Yellow.bold().paint(disk.name().to_string_lossy()),
            (disk.total_space() - disk.available_space()) as f64 / 1e9,
            disk.total_space() as f64 / 1e9
        )
    }
}

fn print_mem(memory_type: &MemType, sys: &System) {
    match memory_type {
        MemType::KB => println!(
            "{} {:.2} KB / {:.2} KB",
            Blue.bold().paint("Memory:"),
            sys.used_memory(),
            sys.total_memory()
        ),
        MemType::MB => {
            println!(
                "{} {:.2} MB / {:.2} MB",
                Blue.bold().paint("Memory:"),
                sys.used_memory() as f64 / 1e3,
                sys.total_memory() as f64 / 1e3
            )
        }
        MemType::GB => {
            println!(
                "{} {:.2} GB / {:.2} GB",
                Blue.bold().paint("Memory:"),
                sys.used_memory() as f64 / 1e6,
                sys.total_memory() as f64 / 1e6
            )
        }
    }
}

fn print_swap(memory_type: &MemType, sys: &System) {
    match memory_type {
        MemType::KB => println!(
            "{} {:.2} KB / {:.2} KB",
            Blue.bold().paint("Swap:"),
            sys.used_swap(),
            sys.total_swap()
        ),
        MemType::MB => {
            println!(
                "{} {:.2} MB / {:.2} MB",
                Blue.bold().paint("Swap:"),
                sys.used_swap() as f64 / 1e3,
                sys.total_swap() as f64 / 1e3
            )
        }
        MemType::GB => {
            println!(
                "{} {:.2} GB / {:.2} GB",
                Blue.bold().paint("Swap:"),
                sys.used_swap() as f64 / 1e6,
                sys.total_swap() as f64 / 1e6
            )
        }
    }
}

fn print_colors(colors_width: &usize) {
    println!(
        "{}{}{}{}",
        Red.on(Red)
            .paint(format!("{:width$}", width = colors_width)),
        Green
            .on(Green)
            .paint(format!("{:width$}", width = colors_width)),
        Blue.on(Blue)
            .paint(format!("{:width$}", width = colors_width)),
        Yellow
            .on(Yellow)
            .paint(format!("{:width$}", width = colors_width))
    );
    println!(
        "{}{}{}{}",
        Black
            .on(Black)
            .paint(format!("{:width$}", width = colors_width)),
        White
            .on(White)
            .paint(format!("{:width$}", width = colors_width)),
        Purple
            .on(Purple)
            .paint(format!("{:width$}", width = colors_width)),
        Cyan.on(Cyan)
            .paint(format!("{:width$}", width = colors_width))
    );
}

fn print_temps() {
    println!();
    println!("{}", Red.bold().paint("Temperature"));
    println!(
        "{}",
        Red.bold().paint(repeat('-').take(20).collect::<String>())
    );

    let components = Components::new_with_refreshed_list();
    for component in &components {
        println!(
            "{}: {}°C",
            Blue.bold().paint(component.label()),
            component.temperature()
        );
    }
    println!();
}

fn print_desktop_environment() {
    if cfg!(target_os = "linux") {
        if let Ok(de) = std::env::var("DESKTOP_SESSION") {
            println!("{} {}", Blue.bold().paint("DE:"), de);
        }
    }
}
