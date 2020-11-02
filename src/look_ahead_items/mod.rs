pub mod items;
pub mod items_builder;
pub mod look_ahead_items;

pub struct ItemsBuilder<T: std::clone::Clone> {
    /// 読み取った文字列。
    characters: Vec<T>,
    /// 先読みする文字数。
    look_ahead_size: usize,
}

#[derive(Clone)]
pub struct Items<T: std::clone::Clone> {
    characters: Vec<T>,
    look_ahead_characters: LookAheadItems<T>,
    /// 先読みする文字数。
    look_ahead_size: usize,
}

#[derive(Clone)]
pub struct LookAheadItems<T: std::clone::Clone> {
    characters: Vec<T>,
    index: usize,
}
