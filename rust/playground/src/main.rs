use regex::bytes::{Regex, RegexBuilder};
use std::fmt::Debug;

#[derive(Debug)]
struct Meme<T> {
    name: T,
}
impl<T> Meme<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Self { name: T::default() }
    }

    pub fn change_name(&mut self, arg: T) {
        self.name = arg;
    }
}
// Use this so we don't forget to turn off unicode and other flags.
macro_rules! byte_regex {
    ($pat:expr) => {
        RegexBuilder::new($pat)
            .unicode(false)
            .swap_greed(true)
            .build()
    };
}
#[allow(unconditional_panic)]
fn main() {
    // let mut meme = Meme::new();
    // meme.change_name("hey there".to_string());
    // println!("{meme:?}");

    // let mut another_meme = Meme::new();
    // Meme::change_name(&mut another_meme, "hur durr".to_string());

    // another_meme.change_name(meme.name);

    // let r = regex::bytes::Regex::new;
    // let r: Regex = byte_regex!(r"(?P<name>duh) hey there \d{2,4}+").unwrap();
    // let m = r.captures(b"duh hey there 22");
    // dbg!(m.unwrap().name("name"));

    // let x = [1,2,3];
    // dbg!(x[8]);

    let r = regex::Regex::new(r"(^|^[^:]+:\/\/|[^\.]+\.)usaa\.com").unwrap();
    dbg!(&r);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_stuff() {
        let a = 8;
        let b = a * 8;
        assert_eq!(b, 64);
    }
}
