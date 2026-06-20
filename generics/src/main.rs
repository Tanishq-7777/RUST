struct Point<T> {
    x:T,
    y:T,
}

struct  Two_Point<T,U> {
    x:T,
    y:U,
}

impl Two_Point<f64,f64> {
    fn inc(&mut self) {
        self.x += 1.0;
        self.y += 1.0;
    }
}

impl<T,U> Two_Point<T,U> {
    fn new(x:T,y:U) -> Self {
        Self { x:x, y:y }
    }
}

impl<T,U> Two_Point<T,U> {
    fn mixup<X,Y>(self, point: Two_Point<X,Y>) -> Two_Point<T,Y> {
        Two_Point {
            x:self.x,
            y:point.y,
        }
    } 
}

fn main() {

    //We have list1 -> i32 
    //We have list2 -> char
    //To find Largest the function is same but still we had to make 2 different function as retur type was different.

    

    let num_list = vec![1,2,3,4,5];
    let result = largest_i32(&num_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'a', 'm' ,'l', 'a'];
    let result = largest_char(&char_list);
    println!("The largest character is {result}");


    //Now using generics only 1 function largest

    let num_list = vec![1,2,3,4,5];
    let result = largest(&num_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'a', 'm' ,'l', 'a'];
    let result = largest(&char_list);
    println!("The largest character is {result}");


    let point = Point {x:12,y:12};
    let point = Point {x:12.5,y:15.5};

    let point = Two_Point {x:20.5,y:12};
    let point = Two_Point {x:12,y:25.5};
    // point.inc(); //! Can not use it here

    let point = Two_Point::new(12, 24.5);

    let mut point = Two_Point {x:12.5,y:25.5};
    point.inc();

    println!("x -> {}",point.x);
    println!("y -> {}",point.y);


    let point1 = Two_Point {x:20.5,y:12};
    let point = Two_Point {x:12,y:25.5};

    let point2 = point.mixup(point1);




    
    
}
fn largest<T :std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for i in list{
        if i > largest {
            largest = i;
        }
    }
    largest
}


fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for i in list{
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item>largest {
            largest = item;
        }
    }
    largest
}
