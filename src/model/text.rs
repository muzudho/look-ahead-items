use crate::model::Character;
use crate::model::Text;

impl Default for Text {
    fn default() -> Self {
        Text {
            characters: Vec::new(),
            index: 0,
        }
    }
}

impl Iterator for Text {
    type Item = Character;

    // ここではイテレーションの流れを`.curr`と`.next`を使用して定義している。
    // 返り値の型は`Option<T>`で、これは:
    //     * `Iterator`が終了した時は`None`を返し、
    //     * そうでなければ`Some`でラップされた値を返す。
    fn next(&mut self) -> Option<Character> {
        if self.index < self.characters.len() {
            let m = self.characters[self.index].clone();
            self.index += 1;
            Some(m)
        } else {
            None
        }
    }
}

impl Text {
    pub fn push(&mut self, character: &Character) {
        self.characters.push(character.clone())
    }
}
