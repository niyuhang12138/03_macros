use macros::EnumFrom;
#[allow(dead_code)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down,
    Left(u32),
    Right(u32, u32),
}

#[allow(dead_code)]
#[derive(Debug)]
struct DirectionUp {
    speed: u32,
}

fn main() {
    let up: Direction = DirectionUp::new(42).into();
    println!("up: {up:?}")
}

impl DirectionUp {
    fn new(speed: u32) -> Self {
        Self { speed }
    }
}
