use std::fs::File;
use std::io::Read;

use std::{collections::HashMap, sync::Arc, time::Duration};

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;

const ROLLOUT_MGR_KV_SCOPE: &str = "rollout";
pub(crate) const DLP_ROLLOUT_FEATURE: &str = "dlp";

#[derive(Default, Deserialize, Serialize, Clone, PartialEq, Debug)]
pub struct RolloutFeature {
    default: f64,
    accounts: Option<HashMap<String, f64>>,
}

#[derive(Debug)]
pub struct RolloutMgr {
    /// the actual storage of the flags
    inner: RwLock<HashMap<String, RolloutFeature>>,
}

impl RolloutMgr {

    fn inner_from_bytes(bytes: &[u8]) -> anyhow::Result<HashMap<String, RolloutFeature>> {
        let ret = from_slice::<HashMap<String, RolloutFeature>>(bytes)?;
        Ok(ret)
    }
}

fn main() {
    let mut f = File::open("actual_copy.txt").unwrap();

    // read into a String, so that you don't need to do the conversion.
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();

    let r = RolloutMgr{ inner: Default::default()};
    *(r.inner.write()) = RolloutMgr::inner_from_bytes(&buffer).unwrap();
}
