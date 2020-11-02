//! I was repeating, and sometimes I wanted to look ahead.  
//! Shifted by chunks instead of one.  
//! 私は繰り返していました、そして時々私は先を見たいと思いました。  
//! 1つではなくチャンクでシフトしました。  

// Publish:
//
// (1) `cargo test`
// (2) `cargo run --example example`
// (3) Open auto-generated log file. I check it.
// (4) Remove the log file.
// (5) Update `README.md`.
// (6) Version up on Cargo.toml.
// (7) `cargo doc --open`
// (8) Comit to Git-hub.
// (9) `cargo publish --dry-run`
// (10) `cargo publish`

pub mod items;
pub mod items_builder;
pub mod look_ahead_items;

/// Create `Items`.  
/// `Items` を作成します。  
pub struct ItemsBuilder<T: std::clone::Clone> {
    /// Created `Items`.  
    /// 読み取った `Items`。  
    items: Vec<T>,
    /// Number of items to look ahead.  
    /// 先読みする項目数。  
    look_ahead_size: usize,
}

/// Multiple items.  
/// 複数の項目。  
#[derive(Clone)]
pub struct Items<T: std::clone::Clone> {
    items: Vec<T>,
    look_ahead_items: LookAheadItems<T>,
    /// 先読みする文字数。
    look_ahead_size: usize,
}

/// Look-ahead items.  
/// 先読みする項目。  
#[derive(Clone)]
pub struct LookAheadItems<T: std::clone::Clone> {
    items: Vec<T>,
    index: usize,
}
