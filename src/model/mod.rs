pub mod character;
pub mod look_ahead_characters;
pub mod text;

#[derive(Clone)]
pub struct LookAheadCharacters {
    characters: Vec<Character>,
    index: usize,
}

pub struct Text {
    characters: Vec<Character>,
    look_ahead_characters: LookAheadCharacters,
    // index: usize,
}

#[derive(Clone)]
pub struct Character {
    character: char,
}
