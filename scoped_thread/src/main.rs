use std::{sync::atomic::AtomicI32, thread};
use rayon::prelude::*;

fn main() {
    //! The problem with the std::thread is that you need to give it the ownership of all the variable you are cusing inside it.
    //! To solve this problem scoped thread came whose lifetime is equal to the scope means the thread will give its output before main function ends or scope ends.
    //! so now there will not be a problem of outlive of lifetime.
    let  counter = AtomicI32::new(0);
    thread::scope(|scope|{
        for i in 0..8 {
            scope.spawn(|| {
                for i in 0..100000 {
                    counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                }
            });
        }
    });
    println!("{:#?}",counter);
    // thread::scope(|scope| {          // <- scope starts here
    // for i in 0..8 {
    //     scope.spawn(|| { ... }); // <- 8 threads, all spawned inside
    // }
    // });                                // <- ALL 8 threads finish here, before this line completes
    // ! use Rayon when you do not know how many threads you need to spawn and rayon will automatically do that for you.
    let range:Vec<i32> = (0..8).collect();
    let result: i32 = range
        .par_iter()
        .map(|_| {
            let mut count = 0;

            for _ in 0..100_000 {
                count += 1;
            }

            count
        })
        .sum();

    println!("{result}");
    
}
