use std::io;

fn main() {
    println!("Welcome to Smart Energy Company(SEC)");
    println!("Enter your total energy consumption (Kwh):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let energy_consumption: f64 = input.trim().parse().expect("Please enter a valid number");

    
    let rate = if energy_consumption > 200.0 {
        30.0
    } else if energy_consumption > 100.0 {
        25.0
    } else {
        20.0
    };

    let cost = energy_consumption * rate;
    println!("-------------------------------");
    println!("Your final cost is: ₦{:.2}", cost);
    println!("-------------------------------");
}
// This program calculates the cost of energy consumption based on user input and predefined rates.