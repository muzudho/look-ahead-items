# look-ahead-items

I was repeating, and sometimes I wanted to look ahead.  
Shifted by chunks instead of one.  
私は繰り返していました、そして時々私は先を見たいと思いました。  
1つではなくチャンクでシフトしました。  

## Examples

```rust
extern crate look_ahead_items;

use look_ahead_items::ItemsBuilder;

fn main() {
    let raw_text = "123abcあいう一二三
456defgえおか四五六";

    let items = ItemsBuilder::default()
        .set_look_ahead_size(4)
        .read(&raw_text.chars().collect())
        .build();

    for look_ahead_items in items {
        println!(
            "* [{}][{}][{}][{}][{}]",
            look_ahead_items,
            if let Some(ch) = look_ahead_items.get(0) {
                ch.to_string()
            } else {
                "".to_string()
            },
            if let Some(ch) = look_ahead_items.get(1) {
                ch.to_string()
            } else {
                "".to_string()
            },
            if let Some(ch) = look_ahead_items.get(2) {
                ch.to_string()
            } else {
                "".to_string()
            },
            if let Some(ch) = look_ahead_items.get(3) {
                ch.to_string()
            } else {
                "".to_string()
            }
        );
    }
}
```

## Run

```shell
cargo run --example example
```

## Output

```plain
* [123a][1][2][3][a]
* [23ab][2][3][a][b]
* [3abc][3][a][b][c]
* [abcあ][a][b][c][あ]
* [bcあい][b][c][あ][い]
* [cあいう][c][あ][い][う]
* [あいう一][あ][い][う][一]
* [いう一二][い][う][一][二]
* [う一二三][う][一][二][三]
* [一二三
][一][二][三][
]
* [二三
4][二][三][
][4]
* [三
45][三][
][4][5]
* [
456][
][4][5][6]
* [456d][4][5][6][d]
* [56de][5][6][d][e]
* [6def][6][d][e][f]
* [defg][d][e][f][g]
* [efgえ][e][f][g][え]
* [fgえお][f][g][え][お]
* [gえおか][g][え][お][か]
* [えおか四][え][お][か][四]
* [おか四五][お][か][四][五]
* [か四五六][か][四][五][六]
* [四五六][四][五][六][]
* [五六][五][六][][]
* [六][六][][][]
```
