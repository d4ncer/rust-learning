enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn new(cents: i32, state: UsState) -> Option<Coin> {
        match cents {
            1 => Some(Coin::Penny),
            5 => Some(Coin::Nickel),
            10 => Some(Coin::Dime),
            25 => Some(Coin::Quarter(state)),
            _ => None,
        }
    }

    fn to_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("woah, the state is {:?}", state);
                25
            }
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let _home = IpAddrKind::V4(127, 0, 0, 1);
    let _loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("i wrote this."));
    m.call();

    let c = Coin::Penny;
    println!("a penny has {} cents", c.to_cents());

    let c = Coin::Nickel;
    println!("a nickel has {} cents", c.to_cents());

    let c = Coin::Dime;
    println!("a dime has {} cents", c.to_cents());

    let c = Coin::Quarter(UsState::Alabama);
    println!("a quarter has {} cents", c.to_cents());

    let nc = Coin::new(5, UsState::Alaska);
    println!("nc is a coin? {:?}", nc);

    let nc = Coin::new(10, UsState::Alabama);
    println!("nc is a coin? {:?}", nc);

    let nc = Coin::new(12, UsState::Alabama);
    println!("nc is a coin? {:?}", nc);

    let five = plus_one(Some(4));
    let none = plus_one(None);

    println!("{:?}, {:?}", five, none);
}
