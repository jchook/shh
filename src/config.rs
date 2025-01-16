use std::env;

/// Config struct to hold the parsed settings
pub struct Config {
    pub verbose: i32,
    pub decibel_threshold: f32,
    pub alert_frequency: u128,
    pub sensitivity: f32,
}

impl Config {
    /// Load the configuration from environment variables
    pub fn load() -> Self {
        Self {
            verbose: get_env_var("SHH_VERBOSE", 0),
            decibel_threshold: get_env_var("SHH_DECIBEL_THRESHOLD", -10.0),
            alert_frequency: get_env_var("SHH_ALERT_FREQUENCY", 1000),
            sensitivity: get_env_var("SHH_SENSITIVITY", 0.8),
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

