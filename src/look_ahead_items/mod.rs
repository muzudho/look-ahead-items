pub mod items;
pub mod items_builder;
pub mod look_ahead_items;

pub struct ItemsBuilder<T: std::clone::Clone> {
    /// 読み取った文字列。
    items: Vec<T>,
    /// 先読みする文字数。
    look_ahead_size: usize,
}

#[derive(Clone)]
pub struct Items<T: std::clone::Clone> {
    items: Vec<T>,
    look_ahead_items: LookAheadItems<T>,
    /// 先読みする文字数。
    look_ahead_size: usize,
}

#[derive(Clone)]
pub struct LookAheadItems<T: std::clone::Clone> {
    items: Vec<T>,
    index: usize,
}
