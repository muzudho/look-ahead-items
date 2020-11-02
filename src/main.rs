mod model;
mod parser;

use crate::parser::CharacterLineP;
use crate::parser::DocumentP;

fn main() {
    println!("Hello, world!");
    let text = "1234abcあいうえおABC
5678defgかきくけこDEFG";
    let character_line = CharacterLineP::read(text);
    for character in character_line {
        println!("{}", character);
    }
    DocumentP::read("1234abc");
}
