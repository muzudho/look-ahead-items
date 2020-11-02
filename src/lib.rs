pub mod character;
pub mod look_ahead_characters;
pub mod text;

#[derive(Clone)]
pub struct Text {
    characters: Vec<Character>,
    look_ahead_characters: LookAheadCharacters,
    /// 先読みする文字数。
    look_ahead_size: usize,
}

#[derive(Clone)]
pub struct LookAheadCharacters {
    characters: Vec<Character>,
    index: usize,
}

#[derive(Clone)]
pub struct Character {
    character: char,
}
