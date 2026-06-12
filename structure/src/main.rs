struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Tanishq@123"),
        email: String::from("saxena@gmail.com"),
        sign_in_count: 1,
    };
    println!("The username is: {}",user1.email);
    user1.email = String::from("someone@gmail.com");
    // let mut user2 =  build_user(String::from("saxenat@gmail.com"), String::from("Saxena@123"));


    //user2
    // let user2 = User {
    //     active: user1.active,
    //     username:user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    // OR Use Struct
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}",user3.username);




    //Tuple Structs
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("{}",black.0);
    let Point(x, y, z) = origin;

}

fn build_user(email: String,username:String) -> User {
    User {
        active:true,
        username,
        email,
        sign_in_count:1,
    }
}
