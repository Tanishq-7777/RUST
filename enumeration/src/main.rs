enum IpAddrKind {
    V4,
    V6,
}
// struct IpAddr {
//     kind : InputAddrKind,
//     address: String,
// } -> this is a bad way to deal with enum 
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
}

enum Messages {
    Quit,
    Messages {x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
impl Messages {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    //  enums give you a way of saying a value is one of a possible set of values. 
    let four= IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
    let home = IpAddr::V4(127,0,0,1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Messages::Write(String::from("hello"));

    m.call();
}
fn route(ip_king :IpAddrKind){}
