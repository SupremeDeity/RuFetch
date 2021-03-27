use sysinfo::{System, SystemExt};

// TODO: Add Gpu, Shell, Terminal, Disk Usage
mod conf;
mod types;
use types::Config;

fn main() {
    // Get the system info
    let sys = System::new_all();

    // Enable color support for WIN10
    #[cfg(windows)]
    let enabled = ansi_term::enable_ansi_support();

    let config = Config::new();

    config.print(&sys);
}
