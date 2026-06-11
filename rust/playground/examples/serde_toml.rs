use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TestConfig {
    thing: u64,
    koochy_koo: String,
    opt: Option<String>,
    nested: Option<Nested>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Nested {
    opt: Option<String>,
}

fn main() {
    let test_config = TestConfig {
        koochy_koo: "santoehusnath".into(),
        thing: 99,
        opt: None,
        nested: None,
    };
    let toml = toml::to_string_pretty(&test_config).unwrap();
    dbg!(toml);
}
