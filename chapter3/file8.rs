use std::env;

fn main() {
    let mut path = env::current_dir().expect("can't access current dir");
    loop {
        println!("{}", path.display());
        if ! path.pop(){
            break;
        }
    }
}