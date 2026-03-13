
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Person {
    username: String,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}
impl Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    println!("user 1 email {}", user1.email);

    // Entire instance is mutable!
    let mut user2 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("otherone@example.com");
    println!("user 2 email {}", user2.email);


    let user3 = build_user("user3@email.com".to_string(), "user3".to_string() );
    println!("user 3 email {}", user3.email);
    // can use struct update syntax to reuse struct values to instantiate a new struct
    let user4: User = User{
        email: String::from("user4@email.com"),
        ..user3
    };

    println!("user 4 email {} {} {}", user4.email, user4.username, user4.active);
    // user3.username produces error because struct update user4 moved user3.username
    // because it is type String which does not have Copy -> therefore it was Moved!
    // println!("user 4 email {} {} {}", user3.email, user3.username, user3.active);

    // Note black and origin even though are tuples with same data types --> ARE DIFFERENT TYPES
    // because they are different tuple structs
    let black = Color(0, 2, 3);
    let origin = Point(0, 0, 0);
    println!("black.1: {}", black.1);

    // tuple structs require you to name type of tuple struct before destructing it.
    // This example shows us calling the Point tuple struct before destructing
    let Point(x, y, z) = origin;

    // can define structs w/ no data in it. Useful to implement trait on some type but dont
    // have any actual data for it
    let subject = AlwaysEqual;

    let rect1 = Rectangle{
        width: 10,
        height: 20,
    };

    println!("rect1 area {}", area(&rect1));

    println!("rect1 is {rect1:?}");

    let rect2 = Rectangle2 {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }

    let sq = Rectangle2::square(30);
    println!("Square {:?}", sq);

    let user1 = Person {
        username: String::from("alice"),
        active: true,
    };
    let user2 = Person {
        username: String::from("bob"),
        ..user1
    };

    println!("{}", user1.username);
}
