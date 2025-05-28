// Sum of all numbers from 1 to 100000000 using multithreading

use std::{
    sync::mpsc,
    thread::{self, spawn},
};


fn main() {
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let producer = tx.clone();
        spawn(move || {
            let mut ans: i64 = 0;
            for j in 0..10000000+1 {
                ans= ans + (i*10000000+j);
            }
            producer.send(ans).unwrap();
        });
    }
    drop(tx); // The original tx variable never drops therefore the rx will keep waiting for it, that's why we drop it explicitly

    let mut ans: i64 = 0;
    for val in rx {
        println!("Recieved value: {}",val);
        ans+= val;
    }
    println!("Sum of 1 to 100000000 = {}", ans);
}
