mod model;
mod parser;

use crate::parser::CharacterLineP;
use crate::parser::DocumentP;

fn main() {
    println!("Hello, world!");
    CharacterLineP::read("1234abc");
    DocumentP::read("1234abc");
}
