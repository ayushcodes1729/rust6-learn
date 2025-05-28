// Using Move closures in rust

use std::thread;

fn main() {
    let v = vec![1,2,3,4,5];

    // The below code doesn't runs because v may go out of scope even before the thread get's spwan and since the thread borrows the vector v it can cause a dangling pointer error that's why we move the ownership of vec v to the macro inside the thread by using move keyword
    // let handle = thread::spawn(|| {
    //     println!("{:?}",v)
    // });
    let handle = thread::spawn(move || {
        println!("{:?}",v)
    });

    // println!("{:?}", v); // Err: borrow of a moved value v

    handle.join().unwrap();
}