#![allow(unused)]
fn main() {
use std::sync::RwLock;

struct MyThing {
    my_str: Rc<str>
}

impl MyThing {
    fn drop(self) {
        println!("I've been dropped! {}", self.my_str)
    }
}

let lock = RwLock::new(MyThing { my_str: Box::from_raw("Hey there fam".into()) });

// many reader locks can be held at once
{
    let r1 = lock.read().unwrap();
    let r2 = lock.read().unwrap();
    assert_eq!(&*r1.my_str, "Hey there fam");
    assert_eq!(&*r2.my_str, "Hey there fam");
} // read locks are dropped at this point

// only one write lock may be held, however
{
    let mut w = lock.write().unwrap();
    *w = MyThing { my_str: Box::new("whoops".into()) } ;
    assert_eq!(&*w.my_str, "whoops");
} // write lock is dropped here
}
