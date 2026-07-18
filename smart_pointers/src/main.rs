struct CustomSmartPoniter {
    data : String
}
impl Drop for CustomSmartPoniter {
    fn drop(&mut self) {
        println!("Dropping the custom smart pointer.");
    }
}
fn main() {
    // ! Samrt Pointers allows you to store a value on heap rather than stack.
    let b = Box::new(5); // ? we are storing 5 on the heap and on the stack we are storing the pointer on the location of 5.
    //Where to uyse Box when you dont know size of something at compiletime and you want to use it in a context where you need to know exact size.

    println!("{b} value of b");

    let c = CustomSmartPoniter {
        data : String::from("hey"),
    };
    drop(c);

    let mut d = CustomSmartPoniter {
        data : String::from("hey Tanishq"),
    };
    foo(&mut d);
    println!("main is going to end")
    let x = || {
        5
    }
}
fn foo(sp : &mut CustomSmartPoniter) {
    let x= sp.data;
    println!("{}",x);
}