// Guessing game from the Rust book

use rand::Rng;
use std::cmp::Ordering; // Ordering is an enum
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=101); // 1..101 --> range from 1 to 100 (101 is not included)

    loop { // loop --> infinite loop
        println!("Please input your guess.");

        let mut guess = String::new(); // mut --> mutable variable

        io::stdin() // :: --> associated function (static method)
            .read_line(&mut guess) // & --> reference (immutable by default)
            .expect("Failed to read line"); // expect --> if io::Result is Err, panic! with the message

        let guess: u32 = guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // continue --> go to the next iteration of the loop
        }
        // Two ways to print a variable:
        println!("Secret number is: {secret_number}");
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            // cmp --> compare
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // break --> break the loop
            },
        } // match --> like switch in other languages
    }
}
