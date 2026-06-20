use std::fs::File;

fn main() {
    let r = divide(4,4);

    match r {
        Ok(some) => println!("{some}"),
        Err(some) => println!("{some}"),
    }

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(some) => some,
                Err(err) => panic!("Problem creating the file: {err:?}"),
            },
            _ => {
                panic!("Problem opening the file: {err:?}");
            }

        },
    };
    println!("{greeting_file:?}");

    let greeting_file = File::open("hello.txt").unwrap();
    println!("{greeting_file:?}");

    println!("-----");

    let greeting_file = File::open("helloo.txt").expect("hello sir ji");
    println!("{greeting_file:?}");


    
}
fn divide(x: i32,y: i32) -> Result<i32,String> {
    if y == 0 {
        return Result::Err(String::from("not dividable"));
    }
    return Result::Ok(x/y)
}
