use std::{sync::mpsc, thread};
// Message Passing: Channels

fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("Secret message from thread 1");
        tx.send(val).unwrap()
    });

    let handle = thread::spawn(move || {
        let recieved = rx.recv().unwrap();
        println!("Got: {}", recieved)
    });
    handle.join().unwrap();
}