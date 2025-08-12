enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // something
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}
impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska=> year >= 1959,
            // --snip--
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel=> 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:#?}");
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1), // the 'i' binds to the value in Some
        None => None,
    }
}

fn plus_one_map(x: Option<i32>) -> Option<i32> {
    x.map(|x| x +1)
}

fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let c = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("Value in cents: {c}");

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let five = Some(5);
    let six = five.map(|i| i + 1);
    assert_eq!(six, Some(6));

    let dice_roll = 9;
    match dice_roll {
        3 => add_hat(),
        7 => remove_hat(),
        _ => (),
    }

    let config_max = Some(3u8); // number 3 as an u8
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
}

fn add_hat() {}
fn remove_hat() {}
