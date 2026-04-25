mod macro_ex;

use std::fmt;
use std::ops::Add;
use std::slice;

// global var in rust are called static
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point2 {
    x: i32,
    y: i32,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point2 {}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
fn raw_pointers() {
    // Different from references and smart pointers, raw pointers:
    // Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    // Aren’t guaranteed to point to valid memory
    // Are allowed to be null
    // Don’t implement any automatic cleanup

    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    let address = 0x012345usize;
    let r = address as *const i32;
    println!("r: {:?}", r);

    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

unsafe fn dangerous() {}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    unsafe {
        dangerous();
    }

    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
fn main() {
    // unsafe rust features:
    // Dereference a raw pointer.
    // Call an unsafe function or method.
    // Access or modify a mutable static variable.
    // Implement an unsafe trait.
    // Access fields of unions.
    // raw_pointers();

    let mut input = [1, 2, 3, 4, 5];
    let res = split_at_mut(&mut input, 3);

    println!("Split results {:?}", res);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    println!("value is: {HELLO_WORLD}");

    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }

    // You can use Miri to check & test unsafe code. It's a dynamic tool in Rust's nightly builds

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let milli = Millimeters(1000);
    let meter = Meters(1);

    let res = milli.add(meter);
    println!("Res {:#?}", res);

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

}
