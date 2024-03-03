use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use http::{HeaderMap, HeaderName, HeaderValue};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
pub struct TestConfig {
    pub tcp_socket_path: PathBuf,
    pub udp_socket_path: PathBuf,
    pub ca_file_path: PathBuf,
    pub test_execution_frequency: u64,
    pub metrics_addr: Option<SocketAddr>,
    pub tracing_addr: Option<SocketAddr>,
    pub qs_ops_socket: Option<PathBuf>,
    pub tests: Vec<Test1>,
    pub other_tests: Vec<Test2>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BaseTestParams {
    pub name: String,
    pub account: String,
    pub user_id: Option<String>,
    pub device_id: Option<String>,
    /// Used to track how many times the test failed in a row, not deserialized
    #[serde(default, skip)]
    pub failed_for: i32,
    /// only runs in a specific colo
    pub only_in_colo: Option<u16>,
    /// If `None`, run on every iteration of the loop.
    /// If n, run on every nth iteration.
    pub frequency: Option<u8>,
    /// if set and true, will publish a latency histogram to metrics
    #[serde(default)]
    pub record_latency_metrics: bool,
    /// test must end within this timeout, otherwise fails with error
    pub timeout: Duration,
    /// if set, will use this proxy endpoint as target instead of making a PP reqest
    #[serde(default)]
    pub proxy_endpoint: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Test1 {
    #[serde(flatten)]
    pub base_params: BaseTestParams,
    /// target
    pub url: String,
    /// decision to expect from the proxy. Allow means 200, block means redirect with location
    /// being the block server URL
    pub decision: String,
    /// applies extra headers to the request
    #[serde(deserialize_with = "deser_http_headers", default)]
    pub headers: HeaderMap,
    /// if set and true, http3 request will be skipped and only http1.1 and http2 will run
    #[serde(default)]
    pub skip_http3: bool,
    /// if set, cert verification isn't applied. Only used for PAC, since cert validation for
    /// PAC is complicated
    #[serde(default)]
    pub disable_cert_verification: Option<bool>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct Test2 {
    #[serde(flatten)]
    pub base_params: BaseTestParams,
}

/// Try to deserialize HTTP headers from a TOML table.
fn deser_http_headers<'de, D>(deserializer: D) -> Result<HeaderMap, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;

    // Serde should parse a TOML table from the input.
    let value: toml::Value = serde::Deserialize::deserialize(deserializer)?;
    let toml::Value::Table(table) = value else {
        return Err(D::Error::custom("the 'headers' key must be a table"));
    };

    // Each item in the TOML table should be a HTTP header.
    let hm = table
        .into_iter()
        .map(|(k, v)| {
            let hk = HeaderName::from_lowercase(k.as_bytes()).map_err(D::Error::custom)?;
            let toml::Value::String(s) = v else {
                return Err(D::Error::custom("the 'headers' key must be a table"));
            };
            let hv = HeaderValue::from_bytes(s.as_bytes()).map_err(D::Error::custom)?;
            Ok((hk, hv))
        })
        .collect::<Vec<_>>()
        .into_iter()
        .collect::<Result<Vec<_>, _>>()? // Vec<Result> into Result<Vec> into Vec
        .into_iter()
        .collect();
    Ok(hm)
}

fn main() {
    // let toml = "thing = 88\n";
    let toml = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/config.toml"));
    let test_config: TestConfig = toml::from_str(toml).unwrap();
    dbg!(test_config);
}
