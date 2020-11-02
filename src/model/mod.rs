pub mod character;
pub mod look_ahead_characters;
pub mod text;

pub struct LookAheadCharacters {}

pub struct Text {
    characters: Vec<Character>,
    index: usize,
}

#[derive(Clone)]
pub struct Character {
    character: char,
}
