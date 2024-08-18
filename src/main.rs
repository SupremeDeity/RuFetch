use std::process::{exit, ExitCode};

use sysinfo::System;

// TODO: Add Gpu, Shell, Terminal, Resolution
mod conf;
mod fetch;
mod types;
use types::Config;

fn main() -> ExitCode {
    if !sysinfo::IS_SUPPORTED_SYSTEM {
        println!("This OS isn't supported (yet?).");
        return ExitCode::FAILURE;
    }
    // Get the system info
    let mut sys = System::new_all();

    // Enable color support for WIN10
    #[cfg(windows)]
    let enabled = ansi_term::enable_ansi_support();

    let conf = Config::new();

    fetch::print(&conf, &mut sys);

    return ExitCode::SUCCESS;
}
