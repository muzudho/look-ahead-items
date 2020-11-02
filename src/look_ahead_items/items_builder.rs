use crate::look_ahead_items::{Items, ItemsBuilder, LookAheadItems};

impl<T> Default for ItemsBuilder<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        ItemsBuilder {
            items: Vec::new(),
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
            items: self.items.clone(),
            look_ahead_items: LookAheadItems::new(0, &vec![]),
            look_ahead_size: self.look_ahead_size,
        }
    }
    pub fn set_look_ahead_size<'a>(&'a mut self, size: usize) -> &'a mut Self {
        self.look_ahead_size = size;
        self
    }

    pub fn read<'a>(&'a mut self, items: &Vec<T>) -> &'a Self {
        self.items = items.to_vec();
        self
    }
}
