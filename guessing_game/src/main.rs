use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("The secret number is {}", secret_number);

    loop {
        println!("Input your guess: ");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Please type a number!".yellow());
                continue;
            },
        };

        println!("Your guess is {}", guess);
        
        match guess.cmp(&secret_number){
            Ordering::Less => println!("{}", "Guess is too small".red()),
            Ordering::Greater => println!("{}", "Guess is too big".red()),
            Ordering::Equal => {
                println!("{}", "You Win!".green());
                break;
            },
        }
    }
}
