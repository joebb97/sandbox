use duration_str::deserialize_duration;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    // Use DurationString to handle the string-to-duration conversion
    #[serde(deserialize_with = "deserialize_duration")]
    timeout: Duration,
}

fn main() {
    let json = r#"{"timeout": 5}"#;
    let config: Config = serde_json::from_str(json).unwrap();

    assert_eq!(config.timeout, Duration::from_secs(5));
    println!("Parsed duration: {:?}", config.timeout);
}
