fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("alpha"), 10);
    scores.insert(String::from("beta"), 20);
    let team_name = String::from("alpha");
    let score = scores.get(&team_name).copied().unwrap_or(0);//get returns an Option<&V>
    println!("{score}");
    scores.insert(String::from("alpha"), 20);
    println!("{scores:?}");
    // When anything giving you an Option Unwrap it
    let sc = scores.entry(String::from("beta")).or_insert(30);
    println!("{sc}");
    scores.entry(String::from("gamma")).or_insert(20);
    println!("{scores:?}");

    // if key exist do not do any thing else put a val in tht ky

    for(key,val) in &scores {
        println!("{key}: {val}");
    }

    let first_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(first_name,field_value);
    // println!("{first_name}"); -> because this variable is moved it lost its ownership.




}
