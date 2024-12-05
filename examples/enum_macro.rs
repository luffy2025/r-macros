use anyhow::Result;
use macros::EnumFrom;

#[allow(unused)]
#[derive(Debug, EnumFrom)]
enum Direction<T> {
    Up(DirectionUp<T>),
    Down(DirectionDown), // cannot use u32 directly, because u32 is used in Left(u32), and EnumFrom cannot distinguish them, it will be generated two From<u32> for Left and Down.
    Left(u32),
    Right((u32, u32)),
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionUp<T> {
    x: i32,
    y: i32,
    content: T,
}

#[allow(unused)]
#[derive(Debug)]
struct DirectionDown(u32);

impl<T> DirectionUp<T> {
    fn new(x: i32, y: i32, c: T) -> Self {
        Self { x, y, content: c }
    }
}

fn main() -> Result<()> {
    let up: Direction<String> = DirectionUp::new(1, 2, "abc".into()).into();
    println!("{:?}", up);

    let left: Direction<u32> = 10.into();
    println!("{:?}", left);

    let right: Direction<(i32, i32)> = (20, 30).into();
    println!("{:?}", right);

    let down: Direction<i32> = DirectionDown(40).into();
    println!("{:?}", down);

    Ok(())
}
