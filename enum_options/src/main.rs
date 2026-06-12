// enum Option<T> {
//     None,
//     Some(T),
// }
fn main() {
    let some_number = Option::Some(5);//
    let some_char = Some('e');// Prelude Sum
    let absent_number: Option<i32> = Option::None; // No type mismatch  both side your Option type

  
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None; // Now we are using Prelude Option


    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; -> Problem type mismatch
}
