#[derive(Debug)]
struct Person {
    firstname:String,
    lastname:String,
}
fn main() {
    //In rust closure is very different then Js.
    //Closures are anonymous function for RUST.
    //You can store these function into variables.

    //Below 2 are two examples of closures.

    let cls1 = || println!("hey this is closure 1.");// No curly brakets are req because 1 statement only.

    //In Closures you dont need to specify data type of the function parameter directly it wil autpmatically set type of function parameter from where the function is firstly called.
    let cls2 = |x: i8,y| {
        println!("hey this is clo  sure 2. Value = {x} and Y is = {y}");// You can also usecurly brackets.
    };
    cls1();
    cls2(10,20);//In first time  calling , the data type you will pass for the y will now become the data type for all further call of this closure.

    //closure "add" with no return type specified compiler will automatically set the return  type
    let add = |x,y| {
        x+y
    };
    let result = add(2,4);
    println!("{result}");
    //Just like javascript closures in rust can use the variables defined in their parents scope
    let print_result = || println!("The result is {result}");
    print_result();


    let mut p1 = Person {
        firstname:"Tanishq".to_string(),
        lastname:"Saxena".to_string(),
    };
    
    //to change somthing from closure you need to change closure as mutable.
    let mut change_person = || p1.lastname = "sexy".to_string();
    change_person();
    println!("{p1:?}");

}