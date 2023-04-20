#![allow(unused)] // ignore warnings for unused variables
fn main() {
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _home = IpAddr::V4(127,0,0,1);
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    //option enum ---------------------------
    let x : i8  = 5;
    let y : Option<i8> = Some(5); // enum Option<T>

    //null vs option - something that is not option will always have value
    // let sum = x + y; // not allowed

    //match --------------------------
    value_in_cents(Coin::Quarter(State::California));

    //match must cover all possibilities
    //match with option<>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    //if let - don't have to check all variants 
}

//simple enum
enum IpAddrKind {
    V4,
    V6,
}

//enum with data, every option can have different data
enum IpAddr {
    V4(u8, u8, u8, u8), 
    V6(String),
}

//similar to defining multiple structs
//but we can define functions that takes message
//and it would accept all four variants
enum Message {
    Quit, 
    Move {x : i32, y : i32}, 
    Write(String), 
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        //body
    }
}

#[derive(Debug)]
enum State {
    California, 
    NewYork,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin { //conditional expression based on type
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10, 
        Coin::Quarter(state) =>  {
            println!("State {:#?}!", state); // can access state variable
            25
        }
    }
}

fn plus_one(x : Option<i32>) -> Option<i32> {
    match x {
        None => None, // one arm
        Some(i) => Some(i+1),
    }
}

fn dice_roll() {
    let x = 6;
    match x {
        3 => todo!(), 
        other => todo!(), // all values different from 3
    }
}