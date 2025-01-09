use anyhow::{anyhow, Result};
use macros::my_try;

fn main() -> Result<()> {
    let f1 = my_try!(f1());
    println!("{:?}", f1);
    let f2 = my_try!(f2());
    println!("{:?}", f2);
    let f3 = my_try!(f3());
    println!("{:?}", f3);
    Ok(())
}

fn f1() -> Result<String> {
    Ok("f1".to_string())
}
fn f2() -> Result<String> {
    Ok("f2".to_string())
}
fn f3() -> Result<String> {
    Err(anyhow!("f3".to_string()))
}