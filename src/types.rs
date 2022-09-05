pub use serde::Deserialize;

impl Time {
    pub fn default() -> Self {
        Time::Hour
    }
}

impl MemType {
    pub fn default() -> Self {
        MemType::GB
    }
}

/// Returns the default value for bool fields of [Config]
pub fn default_bool() -> bool {
    true
}

/// Returns false for override fields of [Config]
pub fn bool_false_override() -> bool {
    false
}

/// Returns the default value for usize fields of [Config]
pub fn default_usize() -> usize {
    3
}

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
    #[serde(default = "default_bool")]
    pub show_os: bool,

    #[serde(default = "default_bool")]
    pub show_hostname: bool,

    #[serde(default = "default_bool")]
    pub show_uptime: bool,

    #[serde(default = "default_bool")]
    pub show_kernel_version: bool,

    #[serde(default = "default_bool")]
    pub show_memory: bool,

    #[serde(default = "default_bool")]
    pub show_de: bool,

    #[serde(default = "default_bool")]
    pub show_swap: bool,

    #[serde(default = "default_bool")]
    pub show_colors: bool,

    #[serde(default = "default_bool")]
    pub show_cpu: bool,

    #[serde(default = "default_bool")]
    pub show_cores: bool,

    #[serde(default = "default_bool")]
    pub show_disks: bool,

    #[serde(default = "bool_false_override")]
    pub show_temperature: bool,

    #[serde(default = "default_usize")]
    pub colors_width: usize,

    #[serde(default = "Time::default")]
    pub uptime_type: Time,

    #[serde(default = "MemType::default")]
    pub memory_type: MemType,
}
