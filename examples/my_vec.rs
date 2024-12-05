use anyhow::Result;

fn main() -> Result<()> {
    my_macro!();
    let v = my_vec![1, 2, 3 + 4];
    println!("Vec from my_vec: {:?}", v);

    // we can also use {} or () to create a vector
    let v = my_vec! {1, 2, 3};
    println!("Vec from my_vec: {:?}", v);
    let v = my_vec!(1, 2, 3);
    println!("Vec from my_vec: {:?}", v);

    let v = my_vec![1, 2, 3];
    println!("Vec from my_vec: {:?}", v);

    // rust cannot infer the type of the vector, so we need to specify it
    let v: Vec<i32> = my_vec![];
    println!("Vec from my_vec: {:?}", v);

    let v: Vec<i32> = my_vec![3; 4];
    println!("Vec from my_vec: {:?}", v);

    // into_vec is a function that converts a Box<[T]> into a Vec<T>. it is impl for [T]
    let v1 = <[_]>::into_vec(Box::new([1, 2, 3, 4, 5]));
    println!("Vec from into_vec: {:?}", v1);

    Ok(())
}

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Hello from my_macro!");
    };
}

#[macro_export]
macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem($elem, $n)
    };
    // $(,)? means that the last comma is optional
    ($($x:expr),+ $(,)?) => {
        {
            // let mut temp_vec = Vec::new();
            // $(temp_vec.push($x);)*
            // temp_vec

            // the performance of the above code is not good, because it will allocate memory for each push
            // this is a better way
            <[_]>::into_vec(Box::new([$($x),*]))
        }
    };
}
