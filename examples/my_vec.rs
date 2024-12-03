use macros::{my_macro, my_vec};

fn main() {
    my_macro!();
    let v = my_vec![1, 2, 3];
    println!("Vec from my_vec: {:?}", v);
}
