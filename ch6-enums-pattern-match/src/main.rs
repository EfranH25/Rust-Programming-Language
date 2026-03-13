enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method should be defined here
        println!("called `Message::call()`");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

impl UsState{
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            _ => false,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Hello, world!");

    let four = IpAddr::V4(String::from("127.0.0.1"));
    let six = IpAddr::V6(String::from("::1"));

    let four = IpAddr2::V4(127, 0, 0, 1);
    let six = IpAddr2::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let five = Some(5);
    let size = plus_one(five);
    let none = plus_one(None);


    println!("size: {:?}, none: {:?}", size, none);

    let dice_roll = 9;

    match dice_roll {
        3 => println!("add fancy hat"),
        7 => println!("remove fancy hat"),
        other => println!("move player"),
    }

    // In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    // let state = if let Coin::Quarter(state) = coin {
    //     state
    // } else {
    //     return None;
    // };
    //
    // if state.existed_in(1900) {
    //     Some(format!("{state:?} is pretty old, for America!"))
    // } else {
    //     Some(format!("{state:?} is relatively new."))
    // }
}
