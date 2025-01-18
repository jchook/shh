use std::env;

/// Config struct to hold the parsed settings
pub struct Config {
    pub alert_frequency: u64,
    pub decibel_threshold: f32,
    pub notify: bool,
    pub sensitivity: f32,
    pub verbose: i32,
}

impl Config {
    /// Load the configuration from environment variables
    pub fn load() -> Self {
        Self {
            alert_frequency: get_env_var("SHH_ALERT_FREQUENCY", 1),
            decibel_threshold: get_env_var("SHH_DECIBEL_THRESHOLD", -10.0),
            notify: get_env_var("SHH_NOTIFY", true),
            sensitivity: get_env_var("SHH_SENSITIVITY", 0.8),
            verbose: get_env_var("SHH_VERBOSE", 0),
        }
    }
}

/// Helper function to read environment variables with a default fallback
fn get_env_var<T: std::str::FromStr>(key: &str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|val| val.parse::<T>().ok())
        .unwrap_or(default)
}
