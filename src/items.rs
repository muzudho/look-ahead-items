//! Please iterate.  
//! イテレートしてください。  
use crate::{Items, LookAheadItems};

impl<T> Default for Items<T>
where
    T: std::clone::Clone,
{
    fn default() -> Self {
        Items {
            items: Vec::new(),
            look_ahead_items: LookAheadItems::new(0, &vec![]),
            look_ahead_size: 4,
        }
    }
}

impl<T> Iterator for Items<T>
where
    T: std::clone::Clone,
{
    type Item = LookAheadItems<T>;

    // ここではイテレーションの流れを`.curr`と`.next`を使用して定義している。
    // 返り値の型は`Option<T>`で、これは:
    //     * `Iterator`が終了した時は`None`を返し、
    //     * そうでなければ`Some`でラップされた値を返す。
    fn next(&mut self) -> Option<LookAheadItems<T>> {
        // 先読み。
        let num = self.look_ahead_size;

        if self.look_ahead_items.index < self.items.len() {
            let mut vec = Vec::new();
            for i in self.look_ahead_items.index..self.look_ahead_items.index + num {
                if i < self.items.len() {
                    vec.push(self.items[i].clone());
                }
            }

            let m = LookAheadItems::new(self.look_ahead_items.index, &vec);
            self.look_ahead_items.index += 1;
            Some(m)
        } else {
            None
        }
    }
}

impl<T> Items<T> where T: std::clone::Clone {}
