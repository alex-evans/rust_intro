
use std::collections::HashSet;

fn make_set(words: &str) -> HashSet<&str> {
    words.split_whitespace().collect()
}

fn main() {
    let fruit = make_set("apple orange pear orange");
    let colors = make_set("brown purple orange yellow");

    for c in fruit.intersection(&colors) {
        println!("{:?}", c);
    }

    // println!("{:?}", fruit);
}