#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Hello from my_macro!");
    };
}

#[macro_export]
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
    () => {};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro() {
        my_macro!();
    }

    #[test]
    fn test_my_vec() {
        let v = my_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }
}