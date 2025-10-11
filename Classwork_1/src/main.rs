use std::io;

fn main() {
    println!("Enter temperature in Celsius:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let celsius: f64 = input.trim().parse().expect("Please enter a number");
    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;

    println!("Temperature: {:.1}°C", celsius);
    println!("Converted: {:.1}°F", fahrenheit);
}
// This program reads a temperature in Celsius from the user,
// converts it to Fahrenheit, and prints both values formatted to one decimal place.