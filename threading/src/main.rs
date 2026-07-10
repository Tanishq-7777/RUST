use std::{clone, sync::{Arc, Mutex, atomic::{AtomicI32, Ordering}}, thread::{self, spawn}};
// static mut COUNT: i32 = 0;

fn main() {
    //below  will utilize only 1 core of your CPU
    // let mut x = 0u128;
    // for i in 1..500000000{
    //     x += i;
    // }
    // println!("{x}");
    // let mut x = 0u128;
    // for i in 1..500000000{
    //     x += i;
    // }
    // println!("{x}");


    //Now suppose while this  above code is happening in 1 core of your cpu you want to do other task on another core then spawn a thread
    //input to spawn is a closure
    
    // let thread_fn = || {
    //     let mut x = 0u128;
    //     for i in 1..500000000{
    //         x += i;
    //     }
    //     println!("{x}");
    // };
    // let join_handle = spawn(thread_fn);
    // let join_handle2 = spawn(thread_fn);
    // join_handle2.join();
    // join_handle.join();
    // println!("thread completed")

    // let mut handles = vec![];

    // for i in 0..8 {
    //     handles.push(spawn(move || println!("Hello World {i}")));
    // }
    // for i in handles{
    //     i.join();
    // }
    // for i in 0..100000 {
    //     handles.push(spawn(move || {
    //         unsafe  {
    //             COUNT += 1;
    //         };
    //     }));
    // }
    // for i in handles{
    //     i.join().unwrap();
    // }

    // unsafe {
    //     let count = COUNT;
    // println!("{}", count);
    // };

    //! To create threads
    let join_handle = spawn(|| { println!("Hello World")});
    join_handle.join().expect("Something went wrong");

    // ! This won't compile👇
    // ! Because -> So there are two problems with directly passing &mut count to multiple spawned threads:
        // Multiple threads would require simultaneous mutable references to the same value.
        // The local count might be destroyed while a spawned thread still holds a reference to it.
    // let mut counter = 0;
    // let mut handles = Vec::new();
    // for i in 0..8 {
    //     handles.push(spawn(|| {
    //         for i in 0..100000 {
    //             counter += 1;
    //         }
    //     }))
    // }
    //  so either you make this closure move so that you can pass ownership of this count inside it.


    //One other way to safely increment a variable is Atomic
    // An atomic integer is an integer that can be safely read and modified by multiple threads concurrently without using a Mutex.

    //But again below program won't compile due to same reason as the thread may outlive the current function as after main function counter is finished but thread may be working with counter that is no longer in the memory.
    //to make it work make a static variable as the lifetime of static variable is more than main function.
    // static COUNTER: AtomicI32 = AtomicI32::new(0);
    // let mut handles = Vec::new();
    // for i in 0..8 {
    //     handles.push(spawn(|| {
    //         let mut count = 0;
    //         for i in 0..100000 {
    //             COUNTER.fetch_add(1, Ordering::Relaxed);
    //         }
    //     }))
    // }
    // handles.into_iter().for_each(| h| h.join().unwrap());
    // println!("{COUNTER:?}");


    //? Other way to the same thing with we were doing is using join handles.
    let mut handles = Vec::new();
    for i in 0..8 {
        handles.push(spawn(|| {
            let mut count = 0;
            for i in 0..100000 {
                count += 1;
            }
            count
        }))
    }

    let result:i32 = handles.into_iter().map(| h| h.join().unwrap()).sum();
    println!("{result}");
}
