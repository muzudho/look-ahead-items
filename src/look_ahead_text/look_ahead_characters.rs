use crate::look_ahead_text::LookAheadCharacters;
use std::fmt;

impl LookAheadCharacters {
    pub fn new(index: usize, characters: &Vec<char>) -> Self {
        LookAheadCharacters {
            characters: characters.clone(),
            index: index,
        }
    }

    pub fn get_characters(&self) -> &Vec<char> {
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
