extern crate parsing_model;

use parsing_model::look_ahead_items::ItemsBuilder;

fn main() {
    println!("Hello, world!");
    let raw_text = "1234abcあいうえおABC
5678defgかきくけこDEFG";

    let items = ItemsBuilder::default()
        .set_look_ahead_size(4)
        .read(&raw_text.chars().collect())
        .build();
    for item in items {
        println!("{}", item);
    }
}
