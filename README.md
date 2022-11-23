# RuFetch
A simple, customisable fetch written in Rust. 

![Github Actions](https://github.com/supremedeity/rufetch/actions/workflows/rust.yml/badge.svg)
[![CircleCI Main Branch](https://img.shields.io/circleci/build/gh/SupremeDeity/RuFetch/main?label=main)](https://circleci.com/gh/SupremeDeity/RuFetch/tree/main)
[![CircleCI Dev Branch](https://img.shields.io/circleci/build/gh/SupremeDeity/RuFetch/dev?label=dev)](https://circleci.com/gh/SupremeDeity/RuFetch/tree/dev)
[![Crate Version](https://img.shields.io/crates/v/rufetch?color=green&label=Crate%20version)](https:/crates.io/crates/rufetch)

  
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

| Platform    | Location                                          |
| :---------- | :------------------------------------------------ |
| **Windows** | %appdata%/rufetch/config.toml                     |
| **Linux**   | ~/.config/rufetch/config.toml                     |
| **Mac**     | ~/Library/Application Support/rufetch/config.toml |

## Configuration Options
Following are possible configuration options, their descriptions and their possible values.

| Option                  | Description              | Possible Values        | Default |
| :---------------------- | :----------------------- | :--------------------- | :------ |
| **show_os**             | Show OS                  | `true, false`          | `true`  |
| **show_hostname**       | Show hostname            | `true, false`          | `true`  |
| **show_kernel_version** | Show OS kernel version   | `true, false`          | `true`  |
| **show_memory**         | Show total and used RAM  | `true, false`          | `true`  |
| **show_swap**           | Show total and used Swap | `true, false`          | `true`  |
| **show_uptime**         | Show system uptime       | `true, false`          | `true`  |
| **show_colors**         | Show color blocks        | `true, false`          | `true`  |
| **show_cpu**            | Show CPU usage           | `true, false`          | `true`  |
| **show_cores**          | Show total CPU cores     | `true, false`          | `true`  |
| **show_temperature**    | Show temperature**       | `true, false`          | `false` |
| **show_disks**          | Show Disk name and Usage | `true, false`          | `true`  |
| **uptime_type**         | The uptime's unit        | `Hour, Minute, Second` | `Hour`  |
| **colors_height**       | Height of color blocks** | `any unsigned int`     | `2`     |
| **colors_width**        | Width of color blocks**  | `any unsigned int`     | `2`     |
| **memory_type**         | The memory's unit        | `KB, MB, GB`           | `GB`    |

**Note:** colors_width requires a unsigned integer which is within integer range. Recommended usage is 3 - 5

**Note:** show_temperature currently may not work on all platforms and as such is `false` by default

## License

This program is licensed under [GPL3](https://choosealicense.com/licenses/gpl-3.0/)
