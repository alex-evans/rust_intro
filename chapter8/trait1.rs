trait Show {
    fn show(&self) -> String;
}

impl Show for i32 {
    fn show(&self) -> String {
        format!("four-byte signed {}", self)
    }
}

impl Show for f64 {
    fn show(&self) -> String {
        format!("eight-byte float {}", self)
    }
}

fn main() {
    let answer = Box::new(42);
    let maybe_py = Box::new(3.14);

    let show_list: Vec<Box<dyn Show>> = vec![answer, maybe_py];
    for d in &show_list {
        println!("show {}", d.show());
    }
}
