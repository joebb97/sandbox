use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TestConfig {
    thing: u64,
    opt: Option<String>,
}
fn main() {
    let toml = "thing = 88\n";
    let test_config: TestConfig = toml::from_str(toml).unwrap();
    dbg!(test_config);

    let test_config = TestConfig {
        thing: 99,
        opt: None,
    };
    let toml = toml::to_string(&test_config).unwrap();
    dbg!(toml);
}
