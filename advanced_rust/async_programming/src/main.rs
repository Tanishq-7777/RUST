use reqwest::Response;

#[tokio::main]
async fn main() {
    // let x = foo(); // !future is a promise that whenever this function executes it will return a u32. but futures do nothing unless you await them.
    // ! But await is only allowed inside async block so for that you need to set main function async.
    // ! setting main function async will give you an error
    // ? then how we will ever start an async function

    //Now you can achieve prallel execution using thread as well but switchin between thread is a very expensive task as you need to save the stack of each thread somewhere in the memory.
    //Using async execution you can have multiple task running on a single thread.


    //Suppose you have 1 thread and you have 2 tasks async task so nwo task 1 will run until await and after that it will stop and wait for somthing but it will not block our  thread now wil will come to task 2 and run till await comes after that task 2 will also wait for somthing and still our main thread is not blocked as if it has  any other task to do it  will do it or else it will come to task1 if its result has come.

    // We will use tokio as are async runtime as we do not have any inbuilt runtime in rust.

    let fut = foo().await;
    println!("{fut}");
    let url_1 = "https://www.abes.ac.in/computer-science-engineering.html#faculty-description";
    let url_2 = "https://www.abes.ac.in/cse-aiml.html#faculty-description";
    let res_1 = tokio::spawn(getCse(url_1));
    let res_2 = tokio::spawn(getCse(url_2));
    println!("the second res is {:?}",res_1.await.unwrap().status());
    println!("the second res is {:?}",res_2.await.unwrap().status());
    

}
async fn foo() -> u32 {
    println!("Hey this is foo");
    5
}
async fn getCse( url:&str) -> Response {
    let result = reqwest::get(url).await.unwrap();
    result
}