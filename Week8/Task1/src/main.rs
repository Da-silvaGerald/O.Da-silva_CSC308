use std::process::{Command, Stdio};
use std::io::{Write, Read};
use std::thread;
use std::time::Duration;

fn main() {
    // ---- Child process 1: sleep 5 ----
    let sleep_child = Command::new("sleep")
        .arg("5")
        .spawn()
        .expect("Failed to spawn sleep process");

    println!("Spawned child 1 (sleep 5) with PID: {}", sleep_child.id());

    // ---- Child process 2: ls -la ----
    let ls_child = Command::new("ls")
        .arg("-la")
        .spawn()
        .expect("Failed to spawn ls process");

    println!("Spawned child 2 (ls -la) with PID: {}", ls_child.id());

    // ---- Child process 3: prints Hello from child ----
    let echo_child = Command::new("sh")
        .arg("-c")
        .arg("echo 'Hello from child'")
        .spawn()
        .expect("Failed to spawn echo process");

    println!("Spawned child 3 (echo) with PID: {}", echo_child.id());

    println!("\nYou now have 5 seconds to run:");
    println!("   ps -ef | grep <PID>");
    println!("   pstree -p");
    println!("   top -H");
    println!("\nWaiting so sleep process stays alive...");

    // Give you time to inspect all running processes
    thread::sleep(Duration::from_secs(5));

    println!("Parent done.");
}
