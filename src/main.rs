use sysinfo::{System, SystemExt};

// TODO: Add Gpu, Shell, Terminal, Disk Usage
mod conf;
use conf::Config;

fn main() {
    // Get the system info
    let sys = System::new_all();

    // Enable color support for WIN10
    #[cfg(windows)]
    if cfg!(target_os = "windows") {
        let enabled = ansi_term::enable_ansi_support();
    }

    let config = Config::new();

    config.print_config(&sys);
}
