fn main() {
    let number = 7;
    if number < 7{
        println!("number less than 7");
    }else if number == 7 {
        println!("number eqaul to 7")
    }else {
        println!("condition was false");
    }

    //using if in a let statement
    let condition = true;
    let number1 = if condition {5} else {10};//but remember type of variable should be same 
    // let number1 = if condition {5} else {"10"}; -> this will give error
    println!("the value of numbmer1 is: {number1}");

}
