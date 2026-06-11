fn main() {
    let y = "hey there buddy".to_string();
    let x = 33;
    let g = {
        let g = usize::try_from(x).unwrap() + y.len();
        let mut ret = g;
        if g < 50 {
            ret = 88
        }
        ret
    };
    dbg!(g);

    fn blah(y: &str, x: i64) -> usize {
        usize::try_from(x).unwrap() + y.len()
    }
    dbg!(blah(&y, 33));

    let blah = |x: i64| usize::try_from(x).unwrap() + y.len();
    dbg!(blah(33));
}
