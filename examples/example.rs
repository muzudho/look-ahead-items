extern crate look_ahead_items;

use look_ahead_items::ItemsBuilder;

fn main() {
    let raw_text = "123abcあいう一二三
456defgえおか四五六";

    let items = ItemsBuilder::default()
        .set_look_ahead_size(4)
        .read(&raw_text.chars().collect())
        .build();

    for item in items {
        println!("{}", item);
    }
}
