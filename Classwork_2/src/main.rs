use std::io;

fn main() {
    println!("Welcome to Smart Café!");
    println!("Enter your total bill amount (₦):");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let bill: f64 = input.trim().parse().expect("Please enter a valid number");

    let discount_rate = if bill > 10_000.0 {
        0.15
    } else if bill > 5_000.0 {
        0.10
    } else {
        0.0
    };

    let discount_amount = bill * discount_rate;
    let final_amount = bill - discount_amount;

    println!("-------------------------------");
    println!("Original Bill: ₦{:.2}", bill);
    println!("Discount Applied: {:.2}%", discount_rate * 100.0);
    println!("Final Bill After Discount: ₦{:.2}", final_amount);
    println!("-------------------------------");
}
// This program calculates the final bill amount after applying discounts based on the total bill.