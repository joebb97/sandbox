
fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    take_ownership(s2);
    let s2 = String::from("hello");
    println!("{}, {}", s1, s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    return some_string
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

// fn takes_and_gives_back(a_string: & String) -> & String {
//     return &a_string
// }
