pub mod character;
pub mod character_line;
pub mod look_ahead_characters;

pub struct LookAheadCharacters {}

pub struct CharacterLine {
    characters: Vec<Character>,
}

#[derive(Clone)]
pub struct Character {
    character: char,
}