fn main() {
    // ! Send & Sync traits are used in thread safety. or they are used to represent thread safety at type level.
    // * they are both present in std::marker module.
    let a = 10;

    let get = |b| a + b;
    println!("{}",get(80));

    let mut count = 0;
    let mut increment = || {
        count += 1;
    };
    increment();
    println!("{count}");
    let name = "Tanishq ".to_string();

    let closure = || {
        println!("{name}");
    };
    call_with_fn(closure);

    let mut name = "Tanishq ".to_string();
    let mut closure = || {
        name.push_str("Rustatian ");
        println!("{name}");
    };
    call_with_fn_mut(closure);


    let name = "Tanishq Saxena".to_string();
    let closure = move || {
        println!("{name}");
    };
    closure();
    closure();
    closure();
    call_with_fn_once(closure);
}
fn call_with_fn<T: Fn()>(func :T) {
    func();
    func();
}
fn call_with_fn_mut<T: FnMut()>(mut func :T) {
    func();
    func();
}
fn call_with_fn_once<T: FnOnce()>(func :T) {
    func();
}
