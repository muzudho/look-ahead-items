use crate::model::Character;
use crate::model::CharacterLine;
use crate::parser::CharacterLineP;

impl CharacterLineP {
    pub fn read(line: &str) -> CharacterLine {
        let mut m = CharacterLine::default();
        let ch_vec: Vec<char> = line.chars().collect();
        for (i, ch) in ch_vec.iter().enumerate() {
            m.push(&Character::new(*ch));
        }
        m
    }
}
