use std::{sync::mpsc, thread};
fn main() {
    println!("Hello, world!");
    let (tx,rx) = mpsc::channel();
    let tx = tx.clone();
    thread::spawn(move || {
        let msg = String::from("hey");
        tx.send(msg).unwrap();
        // println!("{msg}");
    });

    let result = rx.recv().unwrap();
    println!("{result}");
}
