use std::f64::consts::PI;
use std::io;

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Circle { radius }
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

fn main() {
    println!("Welcome to our circle struct exercise!");
    println!("Enter the radius of the circle:");

    // Create a new mutable string to hold user input
    let mut input = String::new();

    // Read the input from the terminal
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    // Convert the string input to a floating-point number
    let radius: f64 = input.trim().parse().expect("Please enter a valid number");

    // Create a Circle instance
    let circle = Circle::new(radius);

    // Display results
    println!("Radius: {:.2}", circle.radius);
    println!("Area: {:.2}", circle.area());
    println!("Circumference: {:.2}", circle.circumference());
}
