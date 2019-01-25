// -- ##################################
// -- Task: Using channels to preform save pass of data b/w threads
// -- ##################################

// Use std thread crate
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;

// static value NO_THREADS
static NO_THREADS: i32 = 3;

// Main thread starts
fn main() {
    // Creating endpoints of the channel
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for thread_no in 0..NO_THREADS {
        // Cloning the Sender
        let thread_tx = tx.clone();

        // Sending threads via the channel
        thread::spawn(move || {
            // thread sends the message to the channel
            thread_tx.send(thread_no).unwrap();
            println!("Thread {} finished", thread_no);
        });
    }

    // Collecting all the threads
    let mut thread_holder = Vec::with_capacity(NO_THREADS as usize);
    for _i in 0..NO_THREADS {
        // Get the message from channel
        thread_holder.push(rx.recv());
    }

    // Print the execution order
    println!("{:?}", thread_holder)
}
