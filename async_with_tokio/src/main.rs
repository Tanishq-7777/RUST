use std::task;

use tokio::{select, time::{Duration, sleep}};

#[tokio::main]//this takes this main function and 
async fn main() {
    println!("Hello, world!");
    // test().await;//it will ask tokio executor to exexute it but thread is not blocked
    // test().await;
    //Join will do the concurrency but it will wait unitl last task is finished.
    println!("If you are using join");
    tokio::join!(test(),test());
    //Now tokio spawn will spawn a new task from this async function. and here you will achieve the bect concurrency.
    println!("After using Spawn");
    let t1 = tokio::spawn(test());
    let t2 = tokio::spawn(test());
    let t3 = tokio::spawn(test());
    t1.await;
    t2.await;
    t3.await;
    //select races the future so which one will be finished first it will be executed.
    println!("Using select");
    select! (_ = test()=> {println!("hey this is 1st")},_= test() => println!("Hey this is second"));
}
async fn test() {
    sleep(Duration::from_millis(5000)).await;
    println!("Hello from Tokio");
}
