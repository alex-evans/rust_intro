use std::env;

fn main() {
    let first = env::args().nth(1).expect("please supply an argument");
    let _n: i32 = first.parse().expect("not an integer!");
    
    for arg in std::env::args() {
        println!("'{}'", arg);
    }
}