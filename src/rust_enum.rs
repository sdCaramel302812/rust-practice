enum IpAddrKind {
    V4,
    V6
}

struct IpAddr1 {
    kind: IpAddrKind,
    address: String
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        // do something
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter
}

pub fn rust_enum() {
    ip_example1();
    ip_example2();
    coin_example();
    option_enum();
    match_pattern();
    if_let();
}

fn ip_example1() {
    let home = IpAddr1 {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let look_back = IpAddr1 {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
}

fn ip_example2() {
    let home = IpAddr2::V4(127, 0, 0, 1);
    let look_back = IpAddr2::V6(String::from("::1"));
    let ip = match home {
        IpAddr2::V4(byte1, byte2, byte3, byte4) => (byte1, byte2, byte3, byte4),
        IpAddr2::V6(ip) => (0, 0, 0, 0)
    };
    println!("{:?}", look_back);
    println!("home ip: {:?}", ip);
    println!();
}

fn coin_example() {
    let coin = Coin::Penny;
    let value = match coin {
        Coin::Penny => 1.0,
        Coin::Nickel => 5.0,
        Coin::Dime => 10.0,
        Coin::Quarter => 25.0
    };
    match coin {
        Coin::Quarter => println!("quarter"),
        _ => println!("not quarter")
    }
    println!("the coin is {} dollar", value / 100.0);
    println!();
}

fn option_enum() {
    let six = Some(6);
    let seven = plus_one(six);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn match_pattern() {
    let dice_roll:i32 = 9;
    match dice_roll {
        1 => println!("hello"),
        6 => println!("world"),
        other => println!("{}", other)
    };
    match dice_roll {
        1 => println!("hello"),
        6 => println!("world"),
        _ => println!("don't do anything")
    };
    println!();
}

fn if_let() {                   //  when we only want to excute code for specific variant
    let number = Some(10);
    if let Some(n) = number {
        println!("number is {}", n);
    }
    let coin = Coin::Penny;
    if let Coin::Quarter = coin {
        println!("is qoarter");
    } else if let Coin::Dime = coin {
        println!("is dime");
    } else {
        println!("other coin");
    }
    println!();
}