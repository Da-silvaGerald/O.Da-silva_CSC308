use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    // Start ping (long-running process)
    let mut child = Command::new("ping")
        .arg("google.com")
        .stdout(Stdio::null())
        .spawn()
        .expect("Failed to start ping");

    println!("Spawned ping with PID: {}", child.id());
    println!("Use 'top' or 'ps -ef | grep ping' to locate it now.");
    
    // Wait 5 seconds
    thread::sleep(Duration::from_secs(5));

    // Kill the child process
    println!("Killing ping process...");
    child.kill().expect("Failed to kill child");

    // Wait for process to terminate and fetch exit status
    let status = child
        .wait()
        .expect("Failed to wait on child process");

    println!("Child process exited with status: {:?}", status);
}
