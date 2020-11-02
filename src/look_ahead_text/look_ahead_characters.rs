use crate::look_ahead_text::CharacterTrait;
use crate::look_ahead_text::LookAheadCharacters;
use std::fmt;

impl<T> LookAheadCharacters<T>
where
    T: std::clone::Clone + CharacterTrait,
{
    pub fn new(index: usize, characters: &Vec<T>) -> Self {
        LookAheadCharacters {
            characters: characters.clone(),
            index: index,
        }
    }

    pub fn get_characters(&self) -> &Vec<T> {
        &self.characters
    }
}
impl<T> fmt::Display for LookAheadCharacters<T>
where
    T: std::fmt::Display + CharacterTrait + std::clone::Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for character in &self.characters {
            buf.push_str(&format!("{}", character));
        }
        write!(f, "{}", buf)
    }
}
impl<T> fmt::Debug for LookAheadCharacters<T>
where
    T: std::fmt::Debug + CharacterTrait + std::clone::Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for character in &self.characters {
            buf.push_str(&format!("{:?}", character));
        }
        write!(f, "{}", buf)
    }
}
