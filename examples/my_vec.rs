use anyhow::Result;

fn main() -> Result<()> {
    let v: Vec<i32> = my_vec!["1".parse()?, "2".parse()?, "3".parse()?, "4".parse()?,];
    let v1 = vec![1, 2, 3];
    println!("{v:?}");
    println!("{v1:?}");
    Ok(())
}

#[macro_export]
macro_rules! my_vec {
    () => {Vec::new()};
    ($elem:expr; $n:expr) => {std::vec::from_elem($elem, $n)};
    ($($x:expr), + $(,)?) => {
        <[_]>::into_vec(Box::new([$($x),+]))
    };
}
