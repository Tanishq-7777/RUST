use std::{sync::Mutex, thread};
fn main() {
    let counter = Mutex::new(0);
    thread::scope(|s| {
        for i in 0..100 {
            s.spawn(|| {
            let mut lock = counter.lock().unwrap();
            *lock += 1;
            });
        }
    }); 
    println!("{counter:#?}");
}
