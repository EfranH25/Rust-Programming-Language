//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certainssadasd

use std::ops::Add;
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
fn add_one<T>(x: T) -> T where T: Add<Output = T> + From<i32> + Copy{
    x + T::from(1)
}


fn main() {
    println!("Hello, world!");

    let y = add_one(5.0);
    println!("{y}");
    let y = add_one(5);
    println!("{y}");

}
