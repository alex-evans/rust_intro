
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
    let answer = 42;
    let maybe_py = 3.14;
    let s1 = answer.show();
    let s2 = maybe_py.show();
    println!("show {}", s1);
    println!("show {}", s2);
}