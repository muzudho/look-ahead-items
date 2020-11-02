pub mod look_ahead_characters;
pub mod text;
pub mod text_builder;

pub struct TextBuilder {
    /// 読み取った文字列。
    characters: Vec<char>,
    /// 先読みする文字数。
    look_ahead_size: usize,
}

#[derive(Clone)]
pub struct Text {
    characters: Vec<char>,
    look_ahead_characters: LookAheadCharacters,
    /// 先読みする文字数。
    look_ahead_size: usize,
}

#[derive(Clone)]
pub struct LookAheadCharacters {
    characters: Vec<char>,
    index: usize,
}
