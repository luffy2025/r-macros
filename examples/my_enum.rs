use anyhow::Result;
use macros::EnumFrom;

fn main() -> Result<()> {
    let up: Direction = DirectionUp::new(1, 2).into();
    println!("{:?}", up);

    let left: Direction = 10.into();
    println!("{:?}", left);

    let right: Direction = (20, 30).into();
    println!("{:?}", right);

    let down: Direction = DirectionDown(40).into();
    println!("{:?}", down);

    Ok(())
}

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction {
    Up(DirectionUp),
    Down(DirectionDown), // cannot use u32 directly, because u32 is used in Left(u32), and EnumFrom cannot distinguish them, it will be generated two From<u32> for Left and Down.
    Left(u32),
    Right((u32, u32)),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp {
    x: i32,
    y: i32,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionDown(u32);

impl DirectionUp {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
