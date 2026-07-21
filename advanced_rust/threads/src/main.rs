use std::{sync::{Arc, Mutex}, thread::{self, spawn}};

fn main() {
    let data = vec![1,2,3,4,5];
    // ! Creating a thread will return you a join  handle
    let handle =  thread::spawn(move || {
        println!("Printing the data for thread 1{:?}",data);
    });
    handle.join();
    // A thread needs to have ownership of the data it is working on.
    // ? Why 
    // Remember this handle thread will execute concurrently with main thread.
    //let other_data = data;// ! we moved the ownership of data to some other variable name other_data
    //now after the above statement data will not be valid now if data is pointing to none then how can inside thread it can use data.
    // If you use move keyword and pass ownership of data in the handle now you can not use in other threads.

    // To solve this problem we have concept of Arc -> it is atomic reference count it helps you to share data across multiple thread.

    
    let other_data = Arc::new(vec![1,2,3,4,5]);//ref_count = 1
    let data_for_thread = Arc::clone(&other_data);//ref_count = 2    
    let join_handle =  thread::spawn(move || {
        println!("Printing the data for  thread 2{:?}",data_for_thread);
    });//ref_count = 1
    println!("Printing the data not for thread {:?}",other_data);
    join_handle.join();//wait at this point until thread has finished execution 


    // Now creting multiple threads and mutating the data
    let mut handles = vec![];
    let data = Arc::new(Mutex::new(vec![1,2,3,4,5]));
    for i in  0..4 {
        let shared_data = Arc::clone(&data);
        let handle = spawn(move|| {
            let mut data = shared_data.lock().unwrap();
            data.push(i);
        });
        handles.push(handle);
    }
    for i in handles {
        i.join().unwrap();
    }
    let data = data.lock().unwrap();
    println!("The data after mutating is {:?}",data)

}
