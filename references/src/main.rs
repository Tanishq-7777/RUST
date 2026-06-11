fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}");


    let mut s = String::from("hello");
    change(&mut s);

    let r1 = &mut s;
    // let r2 = &mut s; -> problem can not have 2 or more mut refrence at same time.
    println!("{r1}");
}
fn calculate_length(s :&String) -> usize {
    s.len()
}
fn change(some_string : &mut String) {
    some_string.push_str(", world");

        let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
}
fn dangle() -> String {
    let s = String::from("hello");
    // &s  -> this will give us an error that you can not return a opinter pointing invalid memory 
    // solution is to directly return the string s and give its ownership to someone else;
    s
}
