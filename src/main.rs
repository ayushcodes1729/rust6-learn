// Multithreading

use std::{thread, time::Duration};

fn main() {
    let handle =thread::spawn(|| {
        for i in 1..10 {
            println!("Hi from the new thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // Awaits for the handle to complete and then run the main thread. This prevents interlinked output(parallism). We can get interlinked output if we put this line at the end of the program
    for i in 1..5 {
        println!("Hi from the main thread {}", i);
        thread::sleep(Duration::from_millis(1));

    }
}