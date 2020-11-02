mod model;
mod parser;

use crate::parser::DocumentP;
use crate::parser::TextP;

fn main() {
    println!("Hello, world!");
    let raw_text = "1234abcあいうえおABC
5678defgかきくけこDEFG";
    let text = TextP::read(raw_text);
    for character in text {
        println!("{}", character);
    }
    DocumentP::read("1234abc");
}
