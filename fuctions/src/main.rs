fn main() {
    another_fuction(5);
    print_labeled_measurement(5,'h');
}
fn another_fuction(x:i32){
    println!("The value of x is: {x}");
}
fn print_labeled_measurement(value:i32 , unit_label:char){
    println!("The measurement is: {value}{unit_label}");
    let mut x = 10;
    let mut y = 15;

    // below line will give you error and y = 20 is a statement and not return any value so you have nothing to bind with x.
    //x = y = 20 
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    //statement vs expresssion
    //statement will return you something expressio will not and if you write a semicolon after expression it will become statement
    let x = plus_one(5);
    println!("the value of x is {x}");
}
fn plus_one(x: i32) -> i32 {
    x+1//this is an expression we are returning.
}