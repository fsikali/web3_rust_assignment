//Program that takes in an input on the terminal and prints out from 0 to the input value

use std::io;

pub fn printing_output() { 
    println!("Input a number:");

    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();

    let value: u32 = match user_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Try again.");
            return;
        }
    };

    println!("Numbers from 0 to {}: ", value);
    for input in 0..=value {
        println!("{}", input);
    }
}