use smol::block_on;
fn main() {
    println!("Hello, world!");
    let num = get_num();
    let num1 = get_num();
    let num2 = get_num();
    let num3 = get_num();
    //calling an async function wil return us a Future now we need an Executor to Execute it

    let num_ouput = block_on(num);  
    println!("{num_ouput}"); 
}
//1st way to make a function anync is put async keyword ahead.
async fn get_num() -> u8 {
    println!("Running async function.");
    8
}
//2nd way to make a function anync is put return type Future.
// fn get_numer() -> impl Future<u8> {
//     //return a future.
// }