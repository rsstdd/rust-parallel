// -- ##################################
// -- Task: Safe usable access across threads for previnting data races
// -- ##################################

// Use std thread crate
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Main thread starts
fn main() {
    // Declaring a Arc type data
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    // Creating 3 threads and implementing lock
    for i in 0..3 {
        let data = data.clone();
        thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data[0] += i;
            println!("Thread id: {:?}", i);
            println!("Data value: {:?}", data[0]);
        });
    }

    thread::sleep(Duration::from_millis(10));
}
