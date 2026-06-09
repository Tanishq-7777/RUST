fn main() {
    //Scaler type -> int float bool char
    //?INTEGER -> u32 -> (0 to 2^32) i32 ->-2^31 to 2^31-1
    let num:i8 = 127;//upto 127 is allowed
    println!("{num}");

    //?Floating opint number
    //default type is f:64 and you can also set it to f:32 and they are signed
    let x = 2.0; // f64
    println!("{x}");
    let y: f32 = 3.0; // f32
    println!("{y}");


    //Boolean type
    let t =  true;
    let f:bool  = false;// with explicit type annotation

    //Character type 
    //characters are unicode in rust means you can write any language in rust or emoji as well

    let c = 'z';
    let z: char = 'Z'; // with explicit type annotation
    let heart_eyed_cat = '😻';



    //Compound type

    //The Tuple type
    //tuple is a collection of different type of data in rustt and it is fixed in size means can no be changed.

    let tup:(i32,i8,f64) = (256,127,12.4);//explicit type conversion
    let tup = (132,-127,23.4);
    
    //accessing any value
    let one_thirty_two = tup.0;
    println!("{one_thirty_two}");

    //destructuring of tuple
    let (x,y,z) = tup;
    println!("{x}");//132
    
    //arrays -> fixed length and same type

    let a = [1,2,3,4,5];
    let a:[i32;5] = [2,3,4,5,6]; // explicit type declaration
    let a = [3;5];//array will contain 5 elemts and all value is 3
    let firstVal = a[0];
    println!("{firstVal}");// 3

    //?If you try to access any INDEX GRETER THAN EQUAL TO ARRAY LENGTH THE CODE WILL PANIC AND IN CPP IT WIL GIVE YOU  GARBAGE VALUE HERE WE SAY RUST IS MORE SAFE.
    //Array — fixed size, known at compile time, lives on the stack. 
    //Vector (Vec<T>) — dynamic size, lives on the heap
}
