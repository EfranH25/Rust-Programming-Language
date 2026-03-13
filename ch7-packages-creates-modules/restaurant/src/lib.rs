mod front_of_house;

pub use crate::front_of_house::hosting;

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::eat_at_restaurant();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    // meal.seasonal_fruit = String::from("blueberries");
}

// use crate::front_of_house::hosting;
// pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant_2() {
    hosting::add_to_waitlist();
}

mod customer {
    use super::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

use std::{cmp::Ordering, collections::HashMap};

// use std::io;
// use std::io::Write;

use std::io::{self, Write};

use std::collections::*;

fn thing() {
    let secret_number = rand::random_range(1..10);
    let map: HashMap<&str, u64> = HashMap::new();
}
