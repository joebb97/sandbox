use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Outer {
    msg: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct Thing {
    field: Option<String>,
    other_field: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "c")]
enum Message {
    Request {
        id: String,
        method: String,
        params: Params,
    },
    Params1 {
        a: i32,
        b: u32,
    },
    Params(Params),
    Response {
        id: String,
        result: String,
    },
}

#[derive(Serialize, Deserialize)]
struct Params {
    a: i32,
    b: u32,
}

fn main() {
    let request = Outer {
        msg: Message::Request {
            id: "thing".into(),
            method: "poop".into(),
            params: Params { a: 88, b: 99 },
        },
    };
    println!("{}", serde_json::to_string(&request).unwrap());

    let other = Message::Params(Params { a: 88, b: 99 });
    println!("{}", serde_json::to_string(&other).unwrap());

    let params = Message::Params1 { a: 88, b: 99 };
    println!("{}", serde_json::to_string(&params).unwrap());

    let thing: Thing = serde_json::from_str(r#"{"other_field": "hey"}"#).unwrap();
    dbg!(&thing);
}
