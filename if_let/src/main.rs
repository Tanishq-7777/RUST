#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self,year:u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(i) => println!("The maximum is configured to be {i}"),
        _ => (),
    }

    //Or use if let syntax for short as we are using _ => () this anoying syntax and we have only one condition

    let config_max = Some(String::from("Hey"));
    if let Some(max) = &config_max {
        println!("The maximum is configured to be {max}");
    }
    println!("{config_max:?}");

    //we can also use else block with if that would behave same as _ => () this expresstino was behaving in match block.

    let coin = Coin::Penny;
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    

    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {state:?}!");
    // } else {
    //     count += 1;
    // }

}

fn describe_state_quarter(coin:Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}
