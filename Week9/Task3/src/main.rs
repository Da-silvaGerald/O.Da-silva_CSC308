use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    loop {
        println!("\n=== Note Taking App ===");
        println!("1. Add a note");
        println!("2. View all notes");
        println!("3. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();

        match choice.trim() {
            "1" => add_note(),
            "2" => view_notes(),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option. Try again."),
        }
    }
}

fn add_note() {
    print!("Enter your note: ");
    io::stdout().flush().unwrap();

    let mut note = String::new();
    io::stdin().read_line(&mut note).unwrap();

    let timestamp = current_timestamp();
    let entry = format!("[{}] {}\n", timestamp, note.trim());

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("notes.txt")
        .unwrap();

    file.write_all(entry.as_bytes()).unwrap();
    println!("Note saved.");
}

fn view_notes() {
    let mut file = match File::open("notes.txt") {
        Ok(f) => f,
        Err(_) => {
            println!("No notes found.");
            return;
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("\n=== Your Notes ===");
    println!("{}", contents);
}

fn current_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();

    let seconds = now.as_secs();
    format!("{}s since UNIX epoch", seconds)
}
// This program is a simple command-line note-taking application. It allows users to add notes with timestamps and view all saved notes stored in a text file.