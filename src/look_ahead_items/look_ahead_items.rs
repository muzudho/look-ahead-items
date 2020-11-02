use crate::look_ahead_items::LookAheadItems;
use std::fmt;

impl<T> LookAheadItems<T>
where
    T: std::clone::Clone,
{
    pub fn new(index: usize, characters: &Vec<T>) -> Self {
        LookAheadItems {
            characters: characters.clone(),
            index: index,
        }
    }

    pub fn get_characters(&self) -> &Vec<T> {
        &self.characters
    }
}
impl<T> fmt::Display for LookAheadItems<T>
where
    T: std::fmt::Display + std::clone::Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for character in &self.characters {
            buf.push_str(&format!("{}", character));
        }
        write!(f, "{}", buf)
    }
}
impl<T> fmt::Debug for LookAheadItems<T>
where
    T: std::fmt::Debug + std::clone::Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for character in &self.characters {
            buf.push_str(&format!("{:?}", character));
        }
        write!(f, "{}", buf)
    }
}
