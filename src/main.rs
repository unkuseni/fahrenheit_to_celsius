// use the input library
use std::io;

fn main() {
    // Create a Temperature Converter

    // Prompt the user to enter a number
    println!("Welcome to the temperature converter. Please enter a number:");

    // Read input from the user
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input as a floating-point number
    let temp: Result<f64, _> = input.trim().parse();

    // Check if parsing was successful
    match temp {
        Ok(temp) => {
            // Convert the temperature to Celsius
            let to_celsius = (temp - 32.0) * 5.0 / 9.0;
            // Limit to 3 decimal places and Display the result
            println!("{} in Fahrenheit is {} in Celsius", temp, format!("{:.3}", to_celsius));
        }
        Err(_) => {
            // Handle invalid input
            println!("Invalid input. Please enter a number!");
        }
    }
}
