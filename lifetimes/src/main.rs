
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // Below the lifetime of r is -> a' and lifetime of x is -> b' and you are assigning the reference of a' to b' this will give you error
    // let r; 
    // {
    //     let x = 5;        
    //     r = &x;    
    // }                     
                          
    // println!("r: {r}");

    // -> the below program is valid
    // let x;
    // let r = 10;
    // x = &r;
    // println!("{x}");


    //The below code will give you an error as s1 and s2 have different lifetime
    // let result;
    // let s1 = String::from("Tanishq");
    // {
    //     let s2 = String::from("Tanishq Saxena");
    //     result = longest(&s1,&s2);//result got a reference of s2 but after this scope s2 is not valid at all hence then result would be pointing to a dengling reference.
    // }
    // println!("Result {result}");

    let result;
    let s1 = String::from("Tanishq");
    let s2 = String::from("Tanishq Saxena");
    result = longest(&s1,&s2);//result got a reference of s2 but after this scope s2 is not valid at all hence then result would be pointing to a dengling reference.
    println!("Result {result}");// Borrow checker will check is the smaller lifetime is still valid.

    let ans;

    let string1 = String::from("Tanishq");
    {
        let string2 = String::from("Tani");
        ans = longest(&string1,&string2);
        println!("{ans}");//here ans life time is same as smallest string lifetime.
    }


    //Now we will only return lifetime of first argument from longest_first function hence the borrow checker will only check that is the lifetime of sirst argument valid.
    let ans;
    let string1 = String::from("Tanishq");
    {
        let string2 = String::from("Tani");
        ans = longest_first(&string1,&string2);
        
    }
    println!("{ans}");

    let excerpt = ImportantExcerpt {
        part:ans,
    };
    println!("{excerpt:?}");



}

//below function does not know the lifetime of  x and y.
// fn longest(x :&str, y: &str) -> &str {
//     if x.len() > y.len() {
//         return x;
//     }
//     y
// }

// ! Same longest function below with lifetime
// ! always the smallest lifetime is assigned to 'a in function like in above main func s2 has smallest life time hence 'a will be of smallest lifetime


// ! if x has a smaller lifetime then the return type il also have same lifetime as x.

fn longest<'a>(x :&'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;  
    }
    y
}

fn longest_first<'a>(x:&'a str,y:&str) -> &'a str {
    x
}
 