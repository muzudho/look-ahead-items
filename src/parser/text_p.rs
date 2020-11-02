use crate::model::Character;
use crate::model::Text;
use crate::parser::TextP;

impl TextP {
    pub fn read(line: &str) -> Text {
        let mut m = Text::default();
        let ch_vec: Vec<char> = line.chars().collect();
        for (i, ch) in ch_vec.iter().enumerate() {
            m.push(&Character::new(*ch));
        }
        m
    }
}
