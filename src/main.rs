mod model;
mod parser;

use crate::parser::CharacterLineP;
use crate::parser::DocumentP;

fn main() {
    println!("Hello, world!");
    let character_line = CharacterLineP::read("1234abc");
    for character in character_line {
        println!("{}", character);
    }
    DocumentP::read("1234abc");
}
