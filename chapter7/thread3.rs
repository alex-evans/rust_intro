
use std::thread;

fn main() {
    let name = "dolly".to_string();
    let t1 = thread::spawn(move || {
        println!("hello {}", name);
    });
    let t2 = thread::spawn(move || {
        println!("goodbye {}", name)
    });
    println!("wait {:?}", t1.join());
    println!("wait {:?}", t2.join());
}
