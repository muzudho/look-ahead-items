use crate::look_ahead_items::Items;
use crate::look_ahead_items::ItemsBuilder;
use crate::look_ahead_items::LookAheadItems;

impl<T> Default for ItemsBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        ItemsBuilder {
            characters: Vec::new(),
            /// 先読みする文字数。
            look_ahead_size: 2,
        }
    }
}

impl<T> ItemsBuilder<T>
where
    T: std::clone::Clone,
{
    pub fn build(&self) -> Items<T>
    where
        T: std::clone::Clone,
    {
        Items {
            characters: self.characters.clone(),
            look_ahead_characters: LookAheadItems::new(0, &vec![]),
            look_ahead_size: self.look_ahead_size,
        }
    }
    pub fn set_look_ahead_size<'a>(&'a mut self, size: usize) -> &'a mut Self {
        self.look_ahead_size = size;
        self
    }

    pub fn read<'a>(&'a mut self, characters: &Vec<T>) -> &'a Self {
        self.characters = characters.to_vec();
        self
    }
}
