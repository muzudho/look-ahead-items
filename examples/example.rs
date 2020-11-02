extern crate parsing_model;

use parsing_model::look_ahead_items::ItemsBuilder;

fn main() {
    println!("Hello, world!");
    let raw_text = "1234abcあいうえおABC
5678defgかきくけこDEFG";
    let characters: Vec<char> = raw_text.chars().collect();

    let text = ItemsBuilder::default()
        .set_look_ahead_size(4)
        .read(&characters)
        .build();
    for character in text {
        println!("{}", character);
    }
}
