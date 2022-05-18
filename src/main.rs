use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::Colorize;

fn main() {
    println!("Guess a number!");
    // Generate random number
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {

        // Creates empty string and Takes a guess
        println!("Please input a number.");
        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Change to int32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!,  Guess again!".red()),
            Ordering::Greater => println!("{}", "Too big!, Guess again!".red()),
            Ordering::Equal => {
                println!("{}", "You win!, Congratulations!".green());
                break;
            },
        }  
    }
}
