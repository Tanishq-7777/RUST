fn main() {
    {// s is not valid here, since it's not yet declared
        let s = "hello";// s is valid from this point forward
        println!("{s}");// do stuff with s
    }// this scope is now over, and s is no longer valid

    let s = String::from("hello");//string from string litral
    println!("{s}");


    //because integer size isknown it will go into stack 
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
    //The Above code is perfectly valid both x and y = 5;

    //Now see the ownership
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("the val of s1: {s1}"); s1 -> is not in memory now 
    println!("{s2}, world!");

    let mut s = String::from("hello");
    s = String::from("ahoy");

    println!("{s}, world!");

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1} s2 = {s2}");



    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                              // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);   
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
