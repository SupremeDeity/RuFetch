# RuFetch

A simple, customisable fetch written in Rust.

![Github Actions](https://github.com/supremedeity/rufetch/actions/workflows/rust.yml/badge.svg)
[![CircleCI Main Branch](https://img.shields.io/circleci/build/gh/SupremeDeity/RuFetch/main?label=main)](https://circleci.com/gh/SupremeDeity/RuFetch/tree/main)
[![CircleCI Dev Branch](https://img.shields.io/circleci/build/gh/SupremeDeity/RuFetch/dev?label=dev)](https://circleci.com/gh/SupremeDeity/RuFetch/tree/dev)
[![Crate Version](https://img.shields.io/crates/v/ru_fetch?color=green&label=Crate%20version)](https://crates.io/crates/ru_fetch)

## Sample

```bash
test@test-pc
------------------------------
Host: mohsin-pc
CPU: Intel(R) Core(TM) i5-3320M CPU @ 2.60GHz (4)
OS: Arch Linux
Kernel Version: 5.15.2-arch1-1
DE: bspwm
Uptime:  0.61 hour(s)
Disk: /dev/sda2 (195.16 GB / 449.57 GB)
Memory: 2.44 GB / 8.24 GB
Swap: 0.59 GB / 8.59 GB

Temperature
--------------------
Ambient: 49°C
Core 0: 65°C
Core 1: 64°C
CPU: 63°C
GPU: 46°C
Package id 0: 65°C
SODIMM: 42°C
CPU: 41.5°C
```

## Configuration Location

**The configuration file needs to be created manually.**

| Platform      | Location                                              |
| :------------ | :---------------------------------------------------- |
| **Windows**   | %appdata%/ru_fetch/config.toml                        |
| **Linux**     | ~/.config/ru_fetch/config.toml                        |
| **Mac**       | ~/Library/Application Support/ru_fetch/config.toml    |

## Configuration Options

Following are possible configuration options, their descriptions and their possible values.

| Option                    | Description                   | Possible Values           |
| :-----------------------  | :------------------------     | :------------------------ |
| **show_os**               | Show OS                       | `true, false`             |
| **show_hostname**         | Show hostname                 | `true, false`             |
| **show_kernel_version**   | Show OS kernel version        | `true, false`             |
| **show_memory**           | Show total and used RAM       | `true, false`             |
| **show_swap**             | Show total and used Swap      | `true, false`             |
| **show_uptime**           | Show system uptime            | `true, false`             |
| **show_colors**           | Show color blocks             | `true, false`             |
| **show_cpu**              | Show CPU usage                | `true, false`             |
| **show_cores**            | Show total CPU cores          | `true, false`             |
| **show_disks**            | Show Disk name and Usage      | `true, false`             |
| **show_de**               | **[Linux]** Show DE name      | `true, false`             |
| **show_temperature**      | Show temperature from sensors | `true, false`             |
| **uptime_type**           | The uptime's unit             | `Hour, Minute, Second`    |
| **colors_width**          | Size of color blocks          | `any unsigned int`        |
| **memory_type**           | The memory's unit             | `KB, MB, GB`              |

**Note:** colors_width requires a unsigned integer which is within integer range. Recommended usage is 3 - 5

### Sample / Default Config
<details>
  <summary>Sample Config</summary>
  
```toml
show_os = true
show_hostname = true
show_uptime = true
show_kernel_version = true
show_memory = true
show_swap = true
show_de = true
show_temperature = true
show_colors = true
show_cpu = true
show_cores = true
show_disks = true
colors_width = 3
uptime_type = "Minute"
memory_type = "GB"
```
  
</details>

## License

This program is licensed under [GPL3](https://choosealicense.com/licenses/gpl-3.0/).
