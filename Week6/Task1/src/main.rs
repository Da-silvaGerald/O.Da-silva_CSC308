use std::io;

fn main() {
    // Closure to calculate factorial
    let factorial = |mut n: u32| -> u32 {
        let mut result = 1;

        while n > 1 {
            result *= n;
            n -= 1;
        }

        result
    };

    // Get user input
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let num: u32 = input.trim().parse().expect("Please enter a valid number");

    // Use the closure
    let result = factorial(num);

    println!("Factorial of {} is {}", num, result);
}
