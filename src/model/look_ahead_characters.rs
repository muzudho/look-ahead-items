use crate::model::look_ahead_characters;
use crate::model::Character;
use crate::model::LookAheadCharacters;
use std::fmt;

impl LookAheadCharacters {
    pub fn new(index: usize, characters: &Vec<Character>) -> Self {
        LookAheadCharacters {
            characters: characters.clone(),
            index: index,
        }
    }

    pub fn get_characters(&self) -> &Vec<Character> {
        &self.characters
    }
}
impl fmt::Display for LookAheadCharacters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for character in &self.characters {
            buf.push_str(&format!("{}", character));
        }
        write!(f, "{}", buf)
    }
}
impl fmt::Debug for LookAheadCharacters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for character in &self.characters {
            buf.push_str(&format!("{:?}", character));
        }
        write!(f, "{}", buf)
    }
}
