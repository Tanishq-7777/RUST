use std::thread::spawn;

fn main() {
    //below  will utilize only 1 core of your CPU
    let mut x = 0u128;
    for i in 1..500000000{
        x += i;
    }
    println!("{x}");
    let mut x = 0u128;
    for i in 1..500000000{
        x += i;
    }
    println!("{x}");


    //Now suppose while this  above code is happening in 1 core of your cpu you want to do other task on another core then spawn a thread
    //input to spawn is a closure
    
    let thread_fn = || {
        let mut x = 0u128;
        for i in 1..500000000{
            x += i;
        }
        println!("{x}");
    };
    let join_handle = spawn(thread_fn);
    let join_handle2 = spawn(thread_fn);
    join_handle2.join();
    join_handle.join();
    println!("thread completed")
    
}
