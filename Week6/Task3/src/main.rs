use std::process::Command;

fn main() {
    let mut child = Command::new("echo")
        .arg("Hello from child process!")
        .spawn()
        .expect("Failed to spawn child process");

    // Wait for the child process to finish
    child.wait().expect("Failed to wait on child");
}
