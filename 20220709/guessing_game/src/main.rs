use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess a number!");

    loop {
        println!("Please input a number:");

        let secret = rand::thread_rng().gen_range(1..=3);
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to capture the input.");

        let input = match input.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Only accept positive number. Please try again...\n");
                continue;
            }
        };

        println!("You guessed {}", input);

        match input.cmp(&secret) {
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too BIG"),
            Ordering::Less => println!("Too small"),
        }

        println!("");
    }
}
