use std::net::{IpAddr, Ipv4Addr};

// Declaring a simple enum
#[derive(Debug)]
enum IPVersion { V4, V6 }

// Using the enum in a Struct
#[derive(Debug)]
struct IPAddress {
    version: IPVersion,
    address: String,
}

// Using only enum to declare IPAddress
#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IpAddress {
    fn verify_ip_version(&self) {
        match &self {
            IpAddress::V4(w, x, y, z) => {
                println!("It's IPv4: '{}.{}.{}.{}'", w, x, y, z)
            },
            IpAddress::V6(ip) => println!("It's IPv6: '{}'", ip),
        }
    }
}

// Enum with different data types and quantities
#[derive(Debug)]
enum Message {
    Exit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Implementing a method to Enum Message
impl Message {
    fn print(&self) {
        // println!("Message is: {:#?}", &self);
        match &self {
            Message::Exit => println!("Exiting!"),
            Message::Move { x, y } => {
                println!("Moving to x:{} and y:{}", x, y)
            },
            Message::Write(text) => println!("Writing: '{}'", text),
            Message::ChangeColor(a, b, c) => {
                println!("Changing color to: ({},{},{})", a, b, c)
            },
        }
    }
}

#[derive(Debug)]
enum State {
    Alabama,
    Alaska,
    // etc...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State)
}

// Using MATCH with Enums and values
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Luck Coin!");
            1
        },
        // Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from state: {:#?}", state);
            25
        },
        _ => 0 // PLACEHOLDER (ignored values)
    }
}

// Using MATCH with Option
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let ipv4 = IPVersion::V4;
    let ipv6 = IPVersion::V6;
    router(&ipv4);
    router(&ipv6);

    // Using Struct with enum
    let local = IPAddress {
        version: ipv4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IPAddress {
        version: ipv6,
        address: String::from("::1"),
    };
    println!("Local: {:#?}", local);
    println!("Loopback: {:#?}", loopback);
    router(&local.version);
    println!("Addresses: {} | {}", local.address, loopback.address);

    // Using only enum
    let local = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));
    println!("Local: {:#?}", local);
    println!("Loopback: {:#?}", loopback);
    local.verify_ip_version();
    loopback.verify_ip_version();

    // Using IpAddr from net package
    println!("{}", IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    let m = Message::Exit;
    m.print();
    let m = Message::Write(String::from("Hello Rust!"));
    m.print();
    let m = Message::Move { x: 10, y: 20 };
    m.print();
    let m = Message::ChangeColor(0, 0, 0);
    m.print();

    // Match with values in Enum
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(State::Alabama));
    value_in_cents(Coin::Quarter(State::Alaska));

    // Match with Option
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:#?}, six: {:#?}, none: {:#?}", five, six, none);

    // Using "if let" instead "Match"
    let some_value_u8 = Some(0u8);
    // FROM this
    match some_value_u8 {
        Some(3) => println!("three"),
        _ => (),
    }
    // TO this
    if let Some(3) = some_value_u8 {
        println!("three");
    }
}

fn router(ip_version: &IPVersion) {
    println!("IP Version is: {:#?}", ip_version)
}
