# look-ahead-items

I was repeating, and sometimes I wanted to look ahead.  
Shifted by chunks instead of one.  
私は繰り返していました、そして時々私は先を見たいと思いました。  
1つではなくチャンクでシフトしました。  

## Examples

```rust
extern crate look_ahead_items;

use look_ahead_items::look_ahead_items::ItemsBuilder;

fn main() {
    let raw_text = "123abcあいう一二三
456defgえおか四五六";

    let items = ItemsBuilder::default()
        .set_look_ahead_size(4)
        .read(&raw_text.chars().collect())
        .build();

    for item in items {
        println!("{}", item);
    }
}
```

## Run

```shell
cargo run --example example
```

## Output

```plain
123a
23ab
3abc
abcあ
bcあい
cあいう
あいう一
いう一二
う一二三
一二三

二三
4
三
45

456
456d
56de
6def
defg
efgえ
fgえお
gえおか
えおか四
おか四五
か四五六
四五六
五六
六
```
