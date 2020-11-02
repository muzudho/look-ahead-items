use crate::look_ahead_items::LookAheadItems;
use std::fmt;

impl<T> LookAheadItems<T>
where
    T: std::clone::Clone,
{
    pub fn new(index: usize, items: &Vec<T>) -> Self {
        LookAheadItems {
            items: items.clone(),
            index: index,
        }
    }

    pub fn get_characters(&self) -> &Vec<T> {
        &self.items
    }
}
impl<T> fmt::Display for LookAheadItems<T>
where
    T: std::fmt::Display + std::clone::Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut buf = String::new();
        for item in &self.items {
            buf.push_str(&format!("{}", item));
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
        for item in &self.items {
            buf.push_str(&format!("{:?}", item));
        }
        write!(f, "{}", buf)
    }
}
