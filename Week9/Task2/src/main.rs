use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    // Spawn 10 threads
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 3;
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print final result
    println!("Final counter value: {}", *counter.lock().unwrap());
}
// This program creates 10 threads, each incrementing a shared counter by 3. The counter is protected by a Mutex to ensure safe concurrent access.