extern crate parsing_model;

use parsing_model::Text;

fn main() {
    println!("Hello, world!");
    let raw_text = "1234abcあいうえおABC
5678defgかきくけこDEFG";
    let text = Text::read(raw_text);
    for character in text {
        println!("{}", character);
    }
}
