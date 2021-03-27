# RuFetch
![Github Actions](https://github.com/supremedeity/rufetch/actions/workflows/rust.yml/badge.svg)
[![CircleCI](https://circleci.com/gh/SupremeDeity/RuFetch.svg?style=svg)](https://circleci.com/gh/SupremeDeity/RuFetch)
![Crate Version](https://img.shields.io/crates/v/ru_fetch?color=green&label=Crate%20version)

A simple, customisable fetch written in Rust.

## Sample
```
user@user-pc
------------------------------
OS: Arch Linux
Host: user-pc
Uptime: 24.37 min(s)
Kernel Version: 5.10.15-arch1-1
Memory: 2.45 GB / 8.05 GB
Swap: 1.02 GB / 8.39 GB
```

## Configuration
This is the default configuration:
```toml
show_os = true
show_hostname = true
show_kernel_version = true
show_memory = true
show_swap = true
show_colors = true
colors_width = 3
uptime_type = "Hour"
memory_type = "GB"
```

### **Configuration locations**
The configuration file (`config.toml`) needs to be manually created. These are platform-specific paths:

**Windows**: `%appdata%/ru_fetch/config.toml`

**Linux**: `~/.config/ru_fetch/config.toml`

**Windows**: `~/Library/Application Support/ru_fetch/config.toml`

### **Available Options**
**show_os**: Whether to show OS or not. 
**Possible Values: true, false.**

**show_hostname**: Whether to show Host name or not. 
**Possible Values: true, false.**

**show_kernel_version**: Whether to show the kernel version or not. 
**Possible Values: true, false.**

**show_memory**: Whether to show total/used memory or not. 
**Possible Values: true, false.**

**show_swap**: Whether to show total/used swap or not. 
**Possible Values: true, false.**

**show_colors**: Whether to show color blocks or not. 
**Possible Values: true, false.**

**colors_width**: Width of each indiviual color block. 
**Possible Values: Unsigned integer (within integer range)**

**uptime_type**: Unit to show uptime in. 
**Possible Values: Hour, Minute, Second.**

**memory_type**: Unit to show memory in.
**Possible Values: KB, MB, GB.**
