fn main() {
    //?variable -> runtime contant -> compiletime
    // let mut x = 5;
    // println!("the value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");
    // const THREE_HOUR_IN_SECONDS: u32 = 3 * 60 * 60;
    // println!("Three Hour is seconds: {THREE_HOUR_IN_SECONDS}");

    //? Shadowing
    let x = 5;
    let x = x +1;
    {
        let x = x*2;
        println!("The value of x in the inner scope is: {x}")
    }
    println!("the value of x is: {x}")

    //? immutable vs const
    // 1. let without mut — immutable but runtime
    // rustlet x = some_function(); // fine, computed at runtime
    // x = 5; // ❌ can't reassign, it's immutable

    // 2. const — immutable AND compile time
    // rustconst MAX: u32 = 5 + 3;     // ✅ compiler knows this
    // const Y: f64 = some_function(); // ❌ not allowed, runtime value
}
