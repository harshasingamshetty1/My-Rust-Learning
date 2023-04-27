use std::io;

fn fahrenheit_to_celsius() {
    println!("\n{}\n", "Fahrenheit to Celsius");
    println!("Enter temperature in °F");

    let mut temp_fahrenheit = String::new();
    io::stdin()
        .read_line(&mut temp_fahrenheit)
        .expect("Failed to read imput");
    let temp_fahrenheit: f64 = temp_fahrenheit
        .trim()
        .parse()
        .expect("Please enter a number");

    let temp_celsius: f64 = (temp_fahrenheit - 32.0) * 5.0 / 9.0;

    println!(
        "{}{} = {}{}",
        temp_fahrenheit.to_string(),
        "°F",
        temp_celsius.to_string(),
        "°C"
    );
}

fn celsius_to_fahrenheit() {
    println!("\n{}\n", "Celsius to Fahrenheit");
    println!("Enter temperature in °C");

    let mut temp_celsius = String::new();
    io::stdin()
        .read_line(&mut temp_celsius)
        .expect("ENter a valid input");
    let temp_celsius: f64 = temp_celsius.trim().parse().expect("Please enter a number");

    let temp_fahrenheit: f64 = temp_celsius * 9.0 / 5.0 + 32.0;

    println!(
        "{}{} = {}{}",
        temp_celsius.to_string(),
        "°C",
        temp_fahrenheit.to_string(),
        "°F"
    );
}

fn main() {
    celsius_to_fahrenheit();
    fahrenheit_to_celsius()
}
