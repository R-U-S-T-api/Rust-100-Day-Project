

use std::io::{self, Write};



/*
0 + 1 = 1
1 + 1 = 2
1 + 2 = 3
2 + 3 = 5
3 + 5 = 8
*/

fn main() {
    println!("Fibonnachi");
    print!("Enter the number of terms you want to generate: ");
    io::stdout()
        .flush()
        .unwrap();
    
    let num_term = match get_input_as_u32() {
        Some(value) => value,
        None => {
            println!("Invalid Input");
            return;
        }
    };

    if num_term == 0 {
        println!("Number of term must be greater than zero ");
        return;
    }

    let sequence = generate_fibonacci(num_term);
    println!("Fibonacci Sequince ({} terms) {:?}", num_term , sequence);

    
}


fn get_input_as_u32() -> Option<u32> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    match input.trim().parse::<u32>() {
        Ok(value) => Some(value),
        Err(_) => None,
    }
}

fn generate_fibonacci(n: u32) -> Vec<u64> {
    let mut sequence = Vec::new();
   if n >= 1 {
       sequence.push(0);
   } 
   if n >= 2 {
       sequence.push(1);
   }

   for i in 2..n {
       let next = sequence[i as usize - 1] + sequence[i  as usize -2];
       sequence.push(next);
   }


   sequence

}



