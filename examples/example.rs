extern crate parsing_model;

use parsing_model::model::Text;
use parsing_model::parser::DocumentP;

fn main() {
    println!("Hello, world!");
    let raw_text = "1234abcあいうえおABC
5678defgかきくけこDEFG";
    let text = Text::read(raw_text);
    for character in text {
        println!("{}", character);
    }
    DocumentP::read("1234abc");
}
