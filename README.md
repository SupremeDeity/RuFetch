# RuFetch
A simple, customisable fetch written in Rust.
![Github Actions](https://github.com/supremedeity/rufetch/actions/workflows/rust.yml/badge.svg)
[![CircleCI Main Branch](https://img.shields.io/circleci/build/gh/SupremeDeity/RuFetch/main?label=main)](https://circleci.com/gh/SupremeDeity/RuFetch/tree/main)
[![CircleCI Dev Branch](https://img.shields.io/circleci/build/gh/SupremeDeity/RuFetch/dev?label=dev)](https://circleci.com/gh/SupremeDeity/RuFetch/tree/dev)
[![Crate Version](https://img.shields.io/crates/v/ru_fetch?color=green&label=Crate%20version)](https:/crates.io/crates/ru_fetch)

  
## Sample

```bash
user@user-pc
------------------------------
OS: Arch Linux
Host: user-pc
Uptime: 24.37 min(s)
Kernel Version: 5.10.15-arch1-1
CPU: Intel(R) Core(TM) i5-3320M CPU @ 2.60GHz (4)
Disk: /dev/sda2 (96.61 GB / 449.57 GB)
Memory: 2.45 GB / 8.05 GB
Swap: 1.02 GB / 8.39 GB
```
## Configuration Location

| Platform      | Location                                              |
| :------------ | :---------------------------------------------------- |
| **Windows**   | %appdata%/ru_fetch/config.toml                        |
| **Linux**     | ~/.config/ru_fetch/config.toml                        |
| **Mac**       | ~/Library/Application Support/ru_fetch/config.toml    |

## Configuration Options
--------------
Following are possible configuration options, their descriptions and their possible values.
| Option                    | Description               | Possible Values           |
| :-----------------------  | :------------------------ | :------------------------ |
| **show_os**               | Show OS                   | `true, false`             |
| **show_hostname**         | Show hostname             | `true, false`             |
| **show_kernel_version**   | Show OS kernel version    | `true, false`             |
| **show_memory**           | Show total and used RAM   | `true, false`             |
| **show_swap**             | Show total and used Swap  | `true, false`             |
| **show_uptime**           | Show system uptime        | `true, false`             |
| **show_colors**           | Show color blocks         | `true, false`             |
| **show_cpu**              | Show CPU usage            | `true, false`             |
| **show_cores**            | Show total CPU cores      | `true, false`             |            
| **show_disks**            | Show Disk name and Usage  | `true, false`             | 
| **uptime_type**           | The uptime's unit         | `Hour, Minute, Second`    |
| **colors_width**          | Size of color blocks      | `any unsigned int`        |
| **memory_type**           | The memory's unit         | `KB, MB, GB`              |

**Note:** colors_width requires a unsigned integer which is within integer range. Recommended usage is 3 - 5
