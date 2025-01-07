//Temperature Converter
// Build a CLI program that:
// 1. Converts temperatures between Celsius, Fahrenheit, and Kelvin.
// 2. Validates user input and displays an error message for invalid data.

use std::io;

fn main() {
    println!("Welcome to the Temperature Converter!");

    loop {
        println!("\nPlease select the conversion type:");
        println!("1. Celsius to Fahrenheit");
        println!("2. Celsius to Kelvin");
        println!("3. Fahrenheit to Celsius");
        println!("4. Fahrenheit to Kelvin");
        println!("5. Kelvin to Celsius");
        println!("6. Kelvin to Fahrenheit");
        println!("7. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number between 1 and 7.");
                continue;
            }
        };

        if choice == 7 {
            println!("Exiting the program. Goodbye!");
            break;
        }

        println!("Enter the temperature to convert:");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature).expect("Failed to read input");
        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number for the temperature.");
                continue;
            }
        };

        let result = match choice {
            1 => celsius_to_fahrenheit(temperature),
            2 => celsius_to_kelvin(temperature),
            3 => fahrenheit_to_celsius(temperature),
            4 => fahrenheit_to_kelvin(temperature),
            5 => kelvin_to_celsius(temperature),
            6 => kelvin_to_fahrenheit(temperature),
            _ => {
                println!("Invalid choice. Please select a valid option.");
                continue;
            }
        };

        println!("Converted temperature: {:.2}", result);
    }
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn celsius_to_kelvin(celsius: f64) -> f64 {
    celsius + 273.15
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn fahrenheit_to_kelvin(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0 + 273.15
}

fn kelvin_to_celsius(kelvin: f64) -> f64 {
    kelvin - 273.15
}

fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    (kelvin - 273.15) * 9.0 / 5.0 + 32.0
}