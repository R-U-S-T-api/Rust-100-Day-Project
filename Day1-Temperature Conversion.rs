#![allow(unused)]
#[warn(unused)]

use std::io::{self, Write};

fn main() {
   println!("Temperature Converter");
   println!("---------------------");
   println!("1.) Celcius to Fahrenheit");
   println!("2.) Fahrenheit to Celcius");
   print!("Select option: (1 or 2): ");
   io::stdout().flush().unwrap();

   let mut choice = String::new();
   io::stdin()
       .read_line(&mut choice)
       .expect("Failed to read input");
    
   let choice: u32 = match choice.trim().parse() {
       Ok(num) => num,
       Err(_) => {
           println!("Invalid choice. Please enter 1 or 2");
           return;
       }
   };

   if choice == 1 {
        celcius_to_fahrenheit();
   } else if choice == 2 {
        fahrenheit_to_celcius();
   } else {
       println!("Invalid choices");
   }

}// end of main fn

fn celcius_to_fahrenheit() {
    print!("Enter Tempereature in Celcius: ");
    io::stdout().flush().unwrap();
    
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read input");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. enter valid number");
            return;
        }
    };

    let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
    println!("{:.2} Celcius is {:.2} Fahrenheit", temp, fahrenheit) ;
     
}// end of celcius_to_fahrenheit
 


fn fahrenheit_to_celcius() {
   print!("Enter Temperature in Fahrenheit: ");
   io::stdout().flush().unwrap();

   let mut temp = String::new();
   io::stdin()
       .read_line(&mut temp)
       .expect("Failed to read input");

   let temp: f64 = match temp.trim().parse() {
       Ok(num) => num,
       Err(_) => {
           println!("Invalid input. Enter valid number");
           return;
       }
   };
   
    let celcius = (temp - 32.00) * 5.00 / 9.00;
    println!("Fahrenheit {:.2} is {:.2} Celcius", temp, celcius);

}
