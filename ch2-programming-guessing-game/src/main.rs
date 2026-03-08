use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {}.", secret_number);

    println!("Guess the number!");
    println!("Please input your guess.");

    let mut attempts = 5;

    loop {

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            } ,
        }

        attempts = attempts - 1;

        if attempts <= 0 {
            println!("You have ran out of attempts!");
            break;
        }
        println!("You have {} attempts left.", attempts);

    }


}
