use std::sync::Arc;

#[derive(Debug)]
struct Val {
    val: String
}

fn main() {
    let mut v = Arc::new(Val { val: "hey".into() });
    let a = Arc::get_mut(&mut v).unwrap();
    a.val = "oops".into();
    let b = Arc::get_mut(&mut v).unwrap();
    b.val = "ayy".into();
    println!("{v:?}");
}
