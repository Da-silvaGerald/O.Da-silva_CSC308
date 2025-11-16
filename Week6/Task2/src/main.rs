fn main() {
    // First 20 natural numbers
    let numbers = (1..=20).collect::<Vec<u32>>();

    // Closure to check even numbers
    let is_even = |n: u32| -> bool {
        n % 2 == 0
    };

    // New vector for even numbers
    let mut evens = Vec::new();

    for num in numbers {
        if is_even(num) {
            evens.push(num);
        }
    }

    println!("Even numbers: {:?}", evens);
}
