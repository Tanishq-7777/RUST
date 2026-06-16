fn main() {
    let mut v1: Vec<i32> = Vec::new();

    //But we also have a vec! macro
  
    let v2 = vec![1, 2, 3];//the value inside has copy trait
    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);
    let v = vec![1, 2, 3, 4, 5];

    let third:i32 = v[2];
    println!("{third}");
  
    println!("The third element is {third}");


    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); -> error Because vectors put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space
    //So above is an immutable reference as it is changing v as well
    println!("The first element is: {first}");

    // iterting over vector
    let mut  v = vec![String::from("hey"),String::from("hello")];
    &v.push(String::from("hhm "));
    v[0].push_str("world");
    //  println!("original: {}", s);
     println!("new: {}", v[0]);
     println!("new: {}", v[1]);


    // for i in &v {
    //     println!("{i}");
    // }
    // println!("{}",v[0]);

    // let mut s = v[0];

    println!("-----------");

    let mut v = vec![String::from("hey"),String::from("hello")];
    // for i in & mut v {
    //     println!("{i}");
    //     v.push(i);
    // }
    let mut l:Vec<&mut String> =Vec::new();
    let mut  s= String::from("hlo");
    let x = &mut s;
    let mut s2 = String::from("haloooo") ;;
    l.push(x);
    let y = &mut s2;
    l[0] = y;
    println!("{}",l[0]);


    let mut v = vec![String::from("hey"),String::from("tani")];
    let third = &mut v[1];
    *third = String::from("hloo");
    println!("{}",third);
    println!("{:?}",v);

    

}
