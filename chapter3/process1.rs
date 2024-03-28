use std::process::Command;

fn main() {
    let status = Command::new("rustc")
        .arg("-v")
        .status()
        .expect("no rustc?");

    println!("cool {} code {}", status.success(), status.code().unwrap());
}