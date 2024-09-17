// Importing the entire module
extern crate mods;

// Using a specific part of an module
use mods::network::server::connect;

// Using some items from a enum
use mods::network::TrafficLight::{Green, Yellow};

// Using all enums
// use mods::network::TrafficLight::*;

fn main() {
    mods::client::connect();
    connect();
    let green = Green;
    let yellow = Yellow;
    println!("{:#?}, {:#?}", green, yellow);
}
