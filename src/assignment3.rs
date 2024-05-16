// Program that takes in an input from the terminal and subtracts, adds, multiplies and divides any numbero

use std::io;

pub fn input_to_output() { 
    println!("Enter two numbers separated by space:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let numbers: Vec<f64> = input.trim().split_whitespace()
                                    .map(|x| x.parse().unwrap_or(0.0))
                                    .collect();

    if numbers.len() != 2 {
        println!("Please enter exactly two numbers");
        return;
    }

    println!("Choose an operation:");
    println!("1. Addition");
    println!("2. Subtraction");
    println!("3. Multiplication");
    println!("4. Division");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let choice: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    let result = match choice {
        1 => numbers[0] + numbers[1],
        2 => numbers[0] - numbers[1],
        3 => numbers[0] * numbers[1],
        4 => {
            if numbers[1] != 0.0 {
                numbers[0] / numbers[1]
            } else {
                println!("Try again: Input not divisible by zero )");
                return;
            }
        },
        _ => {
            println!("Invalid input");
            return;
        }
    };

    println!("The result is: {}", result);
}