use std::thread;

fn main() {
    let x = 1;
    let handle = thread::spawn(move || {
        println!("Hello from spawned thread")
    });

    let join_handle = thread::spawn(move || {
        println!("Hello from second spawned thread");
        17
    });

    println!("Hello from the main thread");

    match join_handle.join() {
        Ok(x) => println!("Second spawned thread returned {}", x);
        Err(_) => println!("Second spawned thread panicked")
    }

    println!{"{:?}", handle.join().unwrap()};
}
