use anyhow::Result;
use macros::my_vec;

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec!["1".parse()?, "2".parse()?, "3".parse()?, "4".parse()?,];
    let v1 = vec![1, 2, 3];
    println!("{v:?}");
    println!("{v1:?}");
    Ok(())
}
