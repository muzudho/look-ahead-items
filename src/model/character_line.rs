use crate::model::Character;
use crate::model::CharacterLine;

impl Default for CharacterLine {
    fn default() -> Self {
        CharacterLine {
            characters: Vec::new(),
        }
    }
}

impl CharacterLine {
    pub fn push(&mut self, character: &Character) {
        self.characters.push(character.clone())
    }
}
