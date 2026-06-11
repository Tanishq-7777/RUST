fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("{word}");
    s.clear();
    println!("{s}");

    let hello = &s[0..5];// or &s[..5] bothare same
    let world = &s[6..11];
    let last = &s[0..];// 0 to last
    let zero_to_last = &s[..];


    println!("{hello}");
    println!("{world}");
    println!("{zero_to_last}");

    let st = "hello world";
    let  s= second(st);
    println!("{s}");

}
fn first_word(s :&String) -> usize {
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
fn second(st :&str) -> &str {
    &st[6..]
}
