use crate::Character;
use std::fmt;

impl Character {
    pub fn new(character: char) -> Self {
        Character {
            character: character,
        }
    }
}
impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.character)
    }
}
impl fmt::Debug for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.character)
    }
}
