use colored::Colorize;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter a number between 1 and 100");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input, not a number.");
                return;
            }
        };
        println!("You guessed : {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("{}", "Perfect. You guessed correct".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too big. Try again.".red()),
            Ordering::Less => println!("{}", "Too small. Try again.".red()),
        }
    }
}
