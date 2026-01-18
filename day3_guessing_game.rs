
#![allow(unused)]

// input output
use std::io::{self,Write};
// for random generate
use rand::Rng;
// for ordering true or false 
use std::cmp::Ordering;


fn main() {

    println!("Welcom to guessing game");
    println!("I am thinking of a number between 1 - 100");
    println!("Can you guess it?");
    

    let secret_number = rand::thread_rng().gen_range(1..=100);
    

    loop {
        
        let mut guess = String::new();

        print!("Input your guess: ");
        // to not go to the next line on your input
        io::stdout()
            .flush()
            .unwrap();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        println!("You guessed: {guess}");


        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! try again"),
            Ordering::Greater => println!("Too big! try again"),
            Ordering::Equal => {
                println!("Congratualtion! you guessed the number");
                break;
            }
        }

    }



}
