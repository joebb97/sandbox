use std::collections::HashMap;

fn main() {
    let account = "another_thing";
    let devices = (1..20).map(|i| i.to_string()).collect::<Vec<_>>();
    let pairs: Vec<((String, String), String)> = devices
        .iter()
        .map(|d| ((account.to_string(), d.clone()), d.clone()))
        .collect();
    let mut map: HashMap<(String, String), String> = HashMap::from_iter(pairs.clone());
    dbg!(&map);

    for pair in pairs.iter() {
        map.remove(&pair.0).unwrap();
    }
    dbg!(&map);
}
