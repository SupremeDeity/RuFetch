use ansi_term::Color::*;
use std::process::Command;
use std::str;
use sysinfo::{System, SystemExt};

/* Supports
* CPU
* GPU
* Username
* Computer Name (Host name)
* OS
* Kernel?
* Uptime
* PC name
*/

fn main() {
    let sys = System::new_all();
    //let enabled = ansi_term::enable_ansi_support();

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

    let os = sys.get_name();
    let host_name = sys.get_host_name();
    //let uptime_sec = sys.get_uptime();
    let uptime_min: f64 = sys.get_uptime() as f64 / 60 as f64;
    //let uptime_hour: f64 = sys.get_uptime() as f64 / 3600 as f64;
    let kernel_ver = sys.get_kernel_version();
    let total_mem = sys.get_total_memory() as f64 / 1e+6;
    let total_swap = sys.get_total_swap() as f64 / 1e+6;
    let used_mem = sys.get_used_memory() as f64 / 1e+6;
    let used_swap = sys.get_used_swap() as f64 / 1e+6;

    if let Some(host_name) = &host_name {
        println!(
            "{}@{}",
            Blue.bold().paint(user),
            Blue.bold().paint(host_name)
        );
    }
    println!("{:-<1$}", "", 30);

    if let Some(os) = &os {
        println!("{} {}", Blue.bold().paint("OS:"), os);
    }

    if let Some(host_name) = &host_name {
        println!("{} {}", Blue.bold().paint("Host:"), *host_name);
    }

    //println!("Uptime: {} sec(s)", uptime_sec);
    println!("{} {:.2} min(s)", Blue.bold().paint("Uptime:"), uptime_min);
    //println!("Uptime: {:.2} hour(s)", uptime_hour);

    if let Some(kernel_ver) = &kernel_ver {
        println!("{} {}", Blue.bold().paint("Kernel Version:"), *kernel_ver);
    }

    println!(
        "{} {:.2} GB / {:.2} GB",
        Blue.bold().paint("Memory:"),
        used_mem,
        total_mem
    );
    println!(
        "{} {:.2} GB / {:.2} GB",
        Blue.bold().paint("Swap:"),
        used_swap,
        total_swap
    );
}
