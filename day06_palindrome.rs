
#![allow(unused)]

use std::io::{self,Write};

fn main() {
   
    //palindrome read backward and forward

    println!("palindrome checker");
    println!("enter a string to check if its a palindrome: ");
    io::stdout()
        .flush()
        .unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let cleaned_input = cleaned_string(&input);

    if cleaned_input.is_empty() {
        println!("Enter a valid string.");
        return;
    }


    if is_palindrome(&cleaned_input){
        println!("{} is palindrome", input.trim());
    } else {
        println!("{} is not palindrome", input.trim());
    }
}



fn cleaned_string(input: &str) -> String {
    input
        .chars() //iterate all
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().to_string())
        // better than .map
        // .flat_map(|c| c.to_lowercase())
        .collect::<String>() 
}



fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}
