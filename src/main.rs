// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate toml;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "args")]
enum Actions {
    Wait(usize),
    Move { x: usize, y: usize },
}

fn main() {
    println!("{}", toml::to_string(&Actions::Wait(5)).unwrap());
    println!("{}", toml::to_string(&Actions::Move { x: 1, y: 1 }).unwrap());
}