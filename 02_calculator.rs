#![allow(unused)]

use std::io::{self,Write};

fn main() {

    println!("Calculator app");
    println!("Expression List");
    println!("+ , - , * , /");
    print!("Enter your expression:");
    io::stdout()
        .flush()
        .unwrap();

    let input = get_user_input();

    let num1: f64 = match input[0].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invlid first number");
            return;
        }
    };

    let operator = &input[1];

    let num2: f64 = match input[2].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invlid second number");
            return;
        }
    };

    let result = match &operator[..] {
        "+" => add(num1,num2),
        "-" => sub(num1,num2),
        "*" => mul(num1,num2),
        "/" => dev(num1,num2),
        _ => {
            println!("invalid operator");
            return;
        }
    };


    println!("{}", result);


    



}

fn get_user_input() -> Vec<String> {

   let mut user_input = String::new();
   io::stdin()
       .read_line(&mut user_input)
       .expect("Failed to read line");

   let token: Vec<String> = user_input
       .trim()
       .split_whitespace()
       .map(String::from)
       .collect();

   if token.len() != 3 {
       println!("invalid input please follow format ex.(3 + 3)");
       return Vec::new();
   }

   token

}


fn add(a: f64 ,b: f64) -> f64 {
    a + b
}


fn sub(a: f64 ,b: f64) -> f64 {
    a - b
}


fn mul(a: f64 ,b: f64) -> f64 {
    a * b
}

fn dev(a:f64 ,b: f64) -> f64 {
    if b == 0.0 {
        println!("Div by zero is not allowed");
        std::process::exit(1);
    }
    a / b
}


