use crate::model::Character;
use crate::model::LookAheadCharacters;
use crate::model::Text;

impl Default for Text {
    fn default() -> Self {
        Text {
            characters: Vec::new(),
            look_ahead_characters: LookAheadCharacters::new(0, &vec![]),
        }
    }
}

/*
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
*/

impl Iterator for Text {
    type Item = LookAheadCharacters;

    // ここではイテレーションの流れを`.curr`と`.next`を使用して定義している。
    // 返り値の型は`Option<T>`で、これは:
    //     * `Iterator`が終了した時は`None`を返し、
    //     * そうでなければ`Some`でラップされた値を返す。
    fn next(&mut self) -> Option<LookAheadCharacters> {
        // 先読みは４つとします。
        let num = 4;

        if self.look_ahead_characters.index < self.characters.len() {
            let mut vec = Vec::new();
            for i in self.look_ahead_characters.index..self.look_ahead_characters.index + num {
                if i < self.characters.len() {
                    vec.push(self.characters[i].clone());
                }
            }

            let m = LookAheadCharacters::new(self.look_ahead_characters.index, &vec);
            self.look_ahead_characters.index += 1;
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
