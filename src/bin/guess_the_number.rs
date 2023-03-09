use rand::Rng;
use std::{cmp::Ordering, io::stdin, u32};

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess: String = String::new();

        stdin().read_line(&mut guess).expect("Error Reading Input.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter number not strings 😡, Write the number you guess:");
                continue;
            }
        };

        println!("Write the number you guess:");
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You are right 🫡");
                break;
            }
            Ordering::Greater => println!("Too big 😲"),
            Ordering::Less => println!("Too Smoll 😒"),
        }
    }
}
