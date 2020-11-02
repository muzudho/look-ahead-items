use crate::look_ahead_text::LookAheadCharacters;
use crate::look_ahead_text::Text;
use crate::look_ahead_text::TextBuilder;

impl Default for TextBuilder {
    fn default() -> Self {
        TextBuilder {
            characters: Vec::new(),
            /// 先読みする文字数。
            look_ahead_size: 2,
        }
    }
}

impl TextBuilder {
    pub fn build(&self) -> Text {
        Text {
            characters: self.characters.clone(),
            look_ahead_characters: LookAheadCharacters::new(0, &vec![]),
            look_ahead_size: self.look_ahead_size,
        }
    }
    pub fn set_look_ahead_size<'a>(&'a mut self, size: usize) -> &'a mut Self {
        self.look_ahead_size = size;
        self
    }

    pub fn read<'a>(&'a mut self, line: &str) -> &'a Self {
        self.characters = line.chars().collect();
        self
    }
}
