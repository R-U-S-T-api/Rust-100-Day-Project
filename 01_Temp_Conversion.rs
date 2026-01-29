
/*

    Temperature Converion
    C = (F - 32) x 5 / 9
    F = (C x (9 / 5)) + 32

    
    

*/

#![allow(unused)]

use std::io::{self,Write};


fn main() {

    println!("---------------------");
    println!("Temperature Converter");
    println!("---------------------");
    println!("");
    println!("Choose on the follow");
    println!("1.) Farenheit to Celcius");
    println!("2.) Celcius to Farenheit");

    print!("User input: ");

    let input = match get_user_input_u8() {
        Some(val) => val,
        None => {
            println!("Invalid input, Enter different choices");
            return;
        }
    };


   if input == 1 {
       f_to_c();

   } else if input == 2 {

       c_to_f();
   } else {
        println!("Invalid choice");
   }


    

}


    fn get_user_input_u8() -> Option<u8> {

        to_flush();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<u8>() {
            Ok(value) => Some(value),
            Err(_) => None
        }
    }


    fn f_to_c() {

        println!("");
        println!("-------------------------------");
        println!("Farenheit to Celcius Conversion");
        println!("-------------------------------");
        println!("");


        print!("Enter Farenheit: ");
        to_flush();
        let farenheit = match get_temperature() {
            Some(val) => val,
            None => {
                println!("Jnvlid number");
                return;
            }
        };

        // C = (F - 32) x 5 / 9
        // F = (C x (9 / 5)) + 32

        let mut results = (farenheit - 32.0) * 5.0/9.0;
        println!("Celcius {:.2}, Farenheiht {}", results, farenheit);


    }

    fn c_to_f() {

        println!("");
        println!("-------------------------------");
        println!("Celcius to Farenheiht Conversion");
        println!("-------------------------------");
        println!("");


        print!("Enter Celcius: ");
        to_flush();
        let celcius = match get_temperature() {
            Some(val) => val,
            None => {
                println!("Jnvlid number");
                return;
            }
        };

        // C = (F - 32) x 5 / 9
        // F = (C x (9 / 5)) + 32

        let mut results = celcius * (9.0 / 5.0) + 32.0;
        println!("Farenheiht {:.2}, celcius {}", results, celcius);


    }

    fn to_flush() {
        io::stdout()
            .flush()
            .unwrap();
    }

    fn get_temperature() -> Option<f64> {
        
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("fail to read temperature ");

        match input.trim().parse::<f64>() {
            Ok(val) => Some(val),
            Err(_) => None
        }

    }


