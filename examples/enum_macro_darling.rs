use macros::EnumFromDarling;
#[allow(dead_code)]
#[derive(Debug, EnumFromDarling)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down,
    Left(u32),
    Right(u32, u32),
}

#[allow(dead_code)]
#[derive(Debug)]
struct DirectionUp<T> {
    speed: T,
}

fn main() {
    let up: Direction<u32> = DirectionUp::new(42).into();
    println!("up: {up:?}")
}

impl<T> DirectionUp<T> {
    fn new(speed: T) -> Self {
        Self { speed }
    }
}
