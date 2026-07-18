use std::ops::AddAssign;

trait Sellable {
    fn price(&self) -> u16;
    fn description(&self) -> String;
}
struct Sword {
    name:String,
    damage:u16,
    swing_time_ms:u16,
}
struct Shield {
    name:String,
    armor:u16,
    block:u16,
}

fn vendor_text_static<T: Sellable>(item :&T) -> String {
    format!("I offer you: {}",item.description())
}
fn main() {
    // You have 2 arrays and you have to compute the sum of both the array
    let my_nums = [1,2,3,4];
    let your_nums = [4,3,2,1];

    //What you will do is that you will create a variable and loop through both the array and store the result
    let mut my_sum = 0;
    for i in 0..my_nums.len(){
        my_sum += my_nums[i];
    }
    let mut your_sum = 0;
    for i in 0..my_nums.len(){
        your_sum += your_nums[i];
    }
    println!("{my_sum}{your_sum}");

    //See your code is redundent and to prevent redundency you can make a function for calculating sum
    let my_sum = calculate_sum(&my_nums);
    let your_sum = calculate_sum(&your_nums);
    println!("{my_sum} {your_sum}");
    // ! But the problem above is that if i make an array of i64 then that function would fail

    // ? For this we have a concept of generics -> Generics allows us to use same code for diffrent data

    //TODO -> Traits are  required to restrict geenric parameters.
    let arr: [i64; 4] = [5,6,7,8];
    let my_sum = calculate_generic_sum(&arr);
    println!("{my_sum}");
    let a = 10;


    // ! There is no inheritance in rust but still rust developers don't miss it why?
    let sword = Sword {
        name:"Sword of warrior".into(),
        damage:18,
        swing_time_ms:20,
    };
    let shield = Shield {
        name:"Shield of warrior".into(),
        armor:30,
        block:18,
    };


}
fn calculate_sum(nums :&[i32]) -> i32 {
    let mut sum = 0;
    for i in 0..nums.len(){
        sum += nums[i];
    }
    sum
}
// ! The below function will not compile -> the reason is Assignment operator and add and asignment operator
// ? The Reason is sum can be of move traits means its ownership could be lost or it can be of copy trait.
// ? but move type is not legal here because we have an immutable reference of nums.
// ? secong reason is not all type have trait of Add and assign eg-> you can't do vec1+ vec2

// fn calculate_generic_sum<T>(nums :&[T]) -> T { ---|
    // let mut sum = nums[0];                        | -> Assignment operator 
//     for i in 1..nums.len(){                       |
//         sum += nums[i];                           | -> Add and Asign operator
//     }                                             |
//     sum                                        ---|
// }
fn calculate_generic_sum<T: Copy + AddAssign>(nums :&[T]) -> T { 
    let mut sum = nums[0];
    for i in 1..nums.len(){                       
        sum += nums[i];                         
    }                                         
    sum                                      
}