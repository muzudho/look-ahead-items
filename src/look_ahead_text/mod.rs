pub mod look_ahead_characters;
pub mod text;
pub mod text_builder;

// trait
pub trait CharacterTrait {
    // fn clone() -> CharacterTrait;
}

pub struct TextBuilder<T: CharacterTrait + std::clone::Clone> {
    /// 読み取った文字列。
    characters: Vec<T>,
    /// 先読みする文字数。
    look_ahead_size: usize,
}

#[derive(Clone)]
pub struct Text<T: CharacterTrait + std::clone::Clone> {
    characters: Vec<T>,
    look_ahead_characters: LookAheadCharacters<T>,
    /// 先読みする文字数。
    look_ahead_size: usize,
}

#[derive(Clone)]
pub struct LookAheadCharacters<T: CharacterTrait + std::clone::Clone> {
    characters: Vec<T>,
    index: usize,
}
