fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter*2;
        }
    };
    println!("The result is {result}");

    //Deambiguity with loop you can also break outer loop in nested loop by default break terminates inner loop;

    let mut count = 0;
    'countign_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'countign_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    //streamlining conditional loops with while
    let mut number = 3;
    while number != 0{
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //print array using for and while

    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}",a[index]);
        index += 1;
    }

    //for loop
    for element in a {
        println!("the value is: {element}");
    }

    //for loop with range
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
