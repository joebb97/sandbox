use regex::Regex;

fn main() {
    let re = Regex::new(r"(?-u)/?accounts/([\w-]+)/devices/t\.([\w-]+)/notifications").unwrap();
    let hay = "accounts/d654737e14c2ec4a6393d8c46230348c/devices/t.95ced77c-908d-11ee-a35d-0a34e4cf2553/notifications";
    dbg!(re.captures(hay));
}
