use std::io;

struct Student {
    name: String,
    score: f32,
}

impl Student {
    // Constructor to create a new student
    fn new(name: String, score: f32) -> Self {
        Student { name, score }
    }

    // Determine if the student passed
    fn has_passed(&self) -> bool {
        self.score >= 65.0
    }

    // Display result
    fn display_result(&self) {
        println!("\n--- Student Result ---");
        println!("Name: {}", self.name);
        println!("Score: {:.2}", self.score);

        if self.has_passed() {
            println!("Status: Passed ");
        } else {
            println!("Status: Failed ");
        }
    }
}

fn main() {
    let mut name_input = String::new();
    let mut score_input = String::new();

    println!("Enter student's name:");
    io::stdin().read_line(&mut name_input).expect("Failed to read name");
    let name = name_input.trim().to_string();

    println!("Enter student's score:");
    io::stdin().read_line(&mut score_input).expect("Failed to read score");

    // Convert input to a number (default 0.0 if invalid)
    let score: f32 = score_input.trim().parse().unwrap_or(0.0);

    // Create student and display result
    let student = Student::new(name, score);
    student.display_result();
}
