use std::io;

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // Create a new account
    fn new(owner: String, balance: f64) -> Self {
        BankAccount { owner, balance }
    }

    // Deposit method
    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
            println!("Deposited {:.2}. New balance: {:.2}", amount, self.balance);
        } else {
            println!("Deposit amount must be positive.");
        }
    }

    // Withdraw method
    fn withdraw(&mut self, amount: f64) {
        if amount <= 0.0 {
            println!("Withdrawal amount must be positive.");
        } else if amount > self.balance {
            println!("Insufficient funds. Current balance: {:.2}", self.balance);
        } else {
            self.balance -= amount;
            println!("Withdrew {:.2}. New balance: {:.2}", amount, self.balance);
        }
    }

    // Check balance
    fn check_balance(&self) {
        println!("Current balance for {}: {:.2}", self.owner, self.balance);
    }
}

fn main() {
    let mut input = String::new();

    // Get user's name
    println!("Enter account owner's name:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let owner = input.trim().to_string();

    // Create a new account with zero balance
    let mut account = BankAccount::new(owner, 0.0);

    loop {
        println!("Welcome to the bank account struct exercise!");
        println!("\n--- Bank Menu ---");
        println!("1. Deposit");
        println!("2. Withdraw");
        println!("3. Check Balance");
        println!("4. Exit");
        println!("Enter your choice: ");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice = input.trim();

        match choice {
            "1" => {
                input.clear();
                println!("Enter deposit amount:");
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let amount: f64 = input.trim().parse().unwrap_or(0.0);
                account.deposit(amount);
            }
            "2" => {
                input.clear();
                println!("Enter withdrawal amount:");
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let amount: f64 = input.trim().parse().unwrap_or(0.0);
                account.withdraw(amount);
            }
            "3" => {
                account.check_balance();
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Try again.");
            }
        }
    }
}
