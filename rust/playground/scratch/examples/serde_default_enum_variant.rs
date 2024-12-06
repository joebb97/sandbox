use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Debug)]
#[serde(tag = "ver")]
enum Message {
    #[serde(rename = "v1")]
    V1(MessageV1),
    #[serde(rename = "v2")]
    V2(MessageV2),
}

impl<'de> Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let v = <serde_json::Value as Deserialize>::deserialize(deserializer)?;

        match v.get("ver") {
            Some(ver) if ver == "v2" => Ok(Message::V2(
                serde_json::from_value(v).map_err(serde::de::Error::custom)?,
            )),
            _ => Ok(Message::V1(
                serde_json::from_value(v).map_err(serde::de::Error::custom)?,
            )),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MessageV1 {
    v1_data: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MessageV2 {
    v2_data: String,
}

fn main() {
    let input = json!({
        "ver": "v2",
        "v1_data": "hey there",
        "v2_data": "suh dude"
    });

    let m: Message = serde_json::from_str(input.to_string().as_ref()).unwrap();
    dbg!(&m);

    let out = serde_json::to_string(&m).unwrap();
    dbg!(&out);
}
