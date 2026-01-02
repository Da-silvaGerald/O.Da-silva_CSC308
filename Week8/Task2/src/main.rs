use std::process::{Command, Stdio};
use std::fs::File;
use std::io::{Read, Write};

fn main() {
    // Spawn the echo command and capture stdout
    let mut child = Command::new("sh")
        .arg("-c")
        .arg("echo \"Rust Process Management\"")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn process");

    // Read the child's stdout
    let mut output = String::new();
    child.stdout
        .as_mut()
        .expect("Failed to capture stdout")
        .read_to_string(&mut output)
        .expect("Failed to read stdout");

    // Write to output.txt
    let mut file = File::create("output.txt").expect("Failed to create output.txt");
    file.write_all(output.as_bytes()).expect("Failed to write to file");

    println!("Output written to output.txt.");
}
