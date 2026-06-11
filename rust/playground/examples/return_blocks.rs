use futures::stream::{StreamExt, FuturesUnordered};

async fn more_things<Closure>(mut closure: Closure)
where
    Closure: AsyncFn(i32) -> i32,
{
    let x = vec![0, 3, 8];
    x.into_iter()
        .map(|a| closure(a))
        .collect::<FuturesUnordered<_>>()
        .collect::<Vec<_>>().await.into_iter()
        .for_each(|x| println!("{x}"));
}
async fn do_things(a: i64, x: i64) -> Result<i64, i64> {
    println!("do things");
    let res = match a {
        10 => {
            println!("0 to 10");
            // let x = Err(8)?;
            return Err(8);
        }
        11 => {
            let b = match x {
                800 => return Ok(18),
                0 => return Ok(234),
                _ => {}
            };
            println!("greater than 10");
            println!("b = {b:?}");
            Ok(88)
        }
        _ => {
            println!("something else");
            Ok(4)
        }
    };
    println!("after match block {res:?}");
    res
}
#[tokio::main]
async fn main() {
    // println!("{:?}", do_things(11, 800).await);
    // println!("{:?}", do_things(10, 0).await);
    more_things(async |a| a + 10).await;
}
