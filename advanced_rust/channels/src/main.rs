use std::{sync::mpsc, thread::spawn, time::{Duration, Instant}};

fn main() {
    //multiple senders single reciever channel
    let (tx,rx) = mpsc::channel::<String>();
    // ! through 1 channel you can only send one kind of data they are used to communicate thread.
    let thread = spawn(move || {
        tx.send("Message 1 from thread 1".to_string()).unwrap();// we unwrap as 1 of the side can be broken so in that case it wil give an Err as it is a result type and we want only Ok case hence unsing unwrap(s)
        tx.send("Message 2 from thread 1".to_string()).unwrap();
    });
    // let msg = rx.recv().unwrap();// it blocks the current execution until it recieves a message. -> if you send no message it will block the thread.
    
    let dur = Duration::from_millis(3000);
    // If you want a timeout and you want to try recieve the data in that timeout.
    // let msg = rx.recv_timeout(dur);
    // match msg {
    //     Ok(msg) => println!("Message from tx is -> {}",msg),
    //     Err(err) => println!("No message before timeout"),
    // }
    for msg in rx {
        println!("Recieved th message -> {msg}");
    }
    thread.join();
}
