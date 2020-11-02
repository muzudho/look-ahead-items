extern crate look_ahead_items;

use look_ahead_items::ItemsBuilder;

fn main() {
    let raw_text = "123abcあいう一二三
456defgえおか四五六";

    let items = ItemsBuilder::default()
        .set_look_ahead_size(4)
        .read(&raw_text.chars().collect())
        .build();

    for look_ahead_items in items {
        println!(
            "* [{}][{}][{}][{}][{}]",
            look_ahead_items,
            if let Some(ch) = look_ahead_items.get(0) {
                ch.to_string()
            } else {
                "".to_string()
            },
            if let Some(ch) = look_ahead_items.get(1) {
                ch.to_string()
            } else {
                "".to_string()
            },
            if let Some(ch) = look_ahead_items.get(2) {
                ch.to_string()
            } else {
                "".to_string()
            },
            if let Some(ch) = look_ahead_items.get(3) {
                ch.to_string()
            } else {
                "".to_string()
            }
        );
    }
}
