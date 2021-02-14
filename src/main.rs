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
    //let uptime_sec = sys.get_uptime();
    let uptime_min: f64 = sys.get_uptime() as f64 / 60 as f64;
    //let uptime_hour: f64 = sys.get_uptime() as f64 / 3600 as f64;
    let host_name = sys.get_host_name();
    let kernel_ver = sys.get_kernel_version();
    let total_mem = sys.get_total_memory();
    let total_swap = sys.get_total_swap();
    let used_mem = sys.get_used_memory();
    let used_swap = sys.get_used_swap();

    if let Some(host_name) = &host_name {
        println!("{}@{}", user, host_name);
    }
    println!("{:-<1$}", "", 30);

    if let Some(os) = &os {
        println!("OS: {}", os);
    }

    //println!("Uptime: {} sec(s)", uptime_sec);
    println!("Uptime: {:.2} min(s)", uptime_min);
    //println!("Uptime: {:.2} hour(s)", uptime_hour);

    if let Some(host_name) = &host_name {
        println!("Host Name: {}", *host_name);
    }

    if let Some(kernel_ver) = &kernel_ver {
        println!("Kernel Version: {}", *kernel_ver);
    }

    println!("Memory: {} / {}", used_mem, total_mem);
    println!("Swap: {} / {}", used_swap, total_swap);
}
