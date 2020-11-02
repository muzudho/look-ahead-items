mod model;
mod parser;

use crate::parser::DocumentP;

fn main() {
    println!("Hello, world!");
    DocumentP.read("1234abc");
}
