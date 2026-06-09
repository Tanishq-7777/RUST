use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Guess The Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("Please input your guess");
        let mut guess = String::new();//String is a type provided by standard lib and this String::new() returns a new instance of String.
        io::stdin().read_line(&mut guess).expect("Failed to read the Number");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) =>  num,
            Err(_) => continue,
        };
        //The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents)
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater =>println!("Too Big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
