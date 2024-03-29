use sysinfo::{System, SystemExt};

// TODO: Add Gpu, Shell, Terminal, Resolution
mod conf;
mod fetch;
mod types;
use types::Config;

fn main() {
    // Get the system info
    let sys = System::new_all();

    // Enable color support for WIN10
    #[cfg(windows)]
    let enabled = ansi_term::enable_ansi_support();

    let conf = Config::new();

    fetch::print(&conf, &sys);
}
