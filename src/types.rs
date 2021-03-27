pub use serde::Deserialize;

#[derive(Deserialize)]
pub enum Time {
    Second,
    Minute,
    Hour,
}

#[derive(Deserialize)]
pub enum MemType {
    KB,
    MB,
    GB,
}

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "Config::default_bool")]
    pub show_os: bool,

    #[serde(default = "Config::default_bool")]
    pub show_hostname: bool,

    #[serde(default = "Config::default_bool")]
    pub show_uptime: bool,

    #[serde(default = "Config::default_bool")]
    pub show_kernel_version: bool,

    #[serde(default = "Config::default_bool")]
    pub show_memory: bool,

    #[serde(default = "Config::default_bool")]
    pub show_swap: bool,

    #[serde(default = "Config::default_bool")]
    pub show_colors: bool,

    #[serde(default = "Config::default_bool")]
    pub show_cpu: bool,

    #[serde(default = "Config::default_bool")]
    pub show_cores: bool,

    #[serde(default = "Config::default_bool")]
    pub show_disks: bool,

    #[serde(default = "Config::default_usize")]
    pub colors_width: usize,

    #[serde(default = "Time::default")]
    pub uptime_type: Time,

    #[serde(default = "MemType::default")]
    pub memory_type: MemType,
}
