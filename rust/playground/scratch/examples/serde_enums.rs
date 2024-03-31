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
#[serde(tag = "type")]
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
    Params(Option<Params>),
    Response {
        id: String,
        result: String,
    },
}

#[derive(Serialize, Deserialize)]
enum Message2 {
    Request {
        id: String,
        method: String,
        params: Params,
    },
    Params1 {
        a: i32,
        b: u32,
    },
    Params(Option<Params>),
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

    //     let other = Message::Params(Some(Params { a: 88, b: 99 }));
    //     println!("{}", serde_json::to_string(&other).unwrap());

    //     let other = Message::Params(None);
    //     println!("{}", serde_json::to_string(&other).unwrap());

    //     let other = Message2::Params(Some(Params { a: 88, b: 99 }));
    //     println!("{}", serde_json::to_string(&other).unwrap());

    //     let other = Message2::Params(None);
    // println!("{}", serde_json::to_string(&other).unwrap());

    let params = Message::Params1 { a: 88, b: 99 };
    println!("{}", serde_json::to_string(&params).unwrap());

    let thing: Thing = serde_json::from_str(r#"{"other_field": "hey"}"#).unwrap();
    dbg!(&thing);

    println!(
        "{}",
        serde_json::to_string(&EgressRulesResp {
            egress_scheme: Some(EgressScheme::NamedEgress(NamedEgressScheme {
                named_egress_pool: "pool".into(),
                enable_udp_forwarding: true
            }))
        })
        .unwrap()
    );
}
