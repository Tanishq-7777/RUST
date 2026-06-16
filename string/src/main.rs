fn main() {
    let s = String::from("hello world");
    let slice = &s[0..5];
    let s1 = "hey";
    let s2 = s1.to_string();
    let hello = String::from("नमस्ते");

    let mut s = String::from("hello");
    let mut s1 = String::from( "bar");
    let r = &s;
    

    s.push_str(&s1); // mutablly borrow s and change it + always take reference of argument
    println!("{s}");


    let s1 = String::from("hey");
    let s2  = String::from("goole");
    let s3 = s1 + &s2;// equals to fn add(self, s: &str) -> String {
    // see ownership of self is req and ref of second is req
    println!("{s3}");

    // add takes ownership of s1,
    // it appends a copy of the contents of s2 to s1,
    // and then it returns back ownership of s1.

    // !   compiler can coerce the &String argument into a &str
    // !  When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..]
    // ! in short reference of &String is equal to &str

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    // let s1 = String::from("hi");
    // let h = s1[0];

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }   




}
