use std::ops::Deref;

enum List {
    Cons(i32, Rc<List>),
    Nil
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

struct MyBox<T>(T);

impl <T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello {name}!")
}


struct CustomSmartPointer  {
    data: String
}

impl Drop for CustomSmartPointer  {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}



fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    // old list using Box
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); //Rc::clone does not make a deep copy. Increases reference count
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    let x = 5;
    let y = &x;
    let z = Box::new(x);
    let a= MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
    assert_eq!(5, *a);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer{
        data: String::from("my stuff")
    };
    let d = CustomSmartPointer{
        data: String::from("other stuff")
    };
    println!("CustomerSmartPointers created!");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main");
}
