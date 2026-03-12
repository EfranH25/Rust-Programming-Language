fn ownership_funcs() {
    let s = String::from("hello");  // s comes into scope

    // s's value moves into the function... and so is no longer valid here
    takes_ownership(s);

    // x comes into scope
    let x = 5;

    makes_copy(x);
    println!("{}", x);
    // Because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward.

} // Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}


fn reference_ex(){
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        println!("r1 is {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("r2 is {}", r2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("mutated some string: {}", some_string);
}
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // this is an error!
    println!("{}, world!", s2); // this is not an error! s1 has been invalidated. data moved to s2

    let mut s = String::from("hello");
    println!("{}, world!", s);
    s = String::from("ahoy"); // s set as "ahoy". "hello" immediately dropped and free mem
    println!("{}, world!", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    ownership_funcs();

    reference_ex();

    let some_str = String::from("hello world");

    let first_word = first_word(&some_str);
    // println!("first_word is {}", first_word);
    s.clear();
    println!("the first word is: {first_word}");
    
}
