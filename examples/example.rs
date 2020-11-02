extern crate parsing_model;

use parsing_model::look_ahead_text::TextBuilder;

fn main() {
    println!("Hello, world!");
    let raw_text = "1234abcあいうえおABC
5678defgかきくけこDEFG";
    let text = TextBuilder::default()
        .set_look_ahead_size(4)
        .read(raw_text)
        .build();
    for character in text {
        println!("{}", character);
    }
}
