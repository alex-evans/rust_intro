
#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool)
}

fn eat_and_dump(v: Value) {
    use Value::*;
    match v {
        Number(n) => println!("number is {}", n),
        Str(s) => println!("string is '{}'", s),
        Bool(b) => println!("boolean is {}", b)
    }
}

fn main() {
    use Value::*;
    let n = Number(2.3);
    let s = Str("hello".to_string());
    let b = Bool(true);

    // println!("n {:?} s {:?} b {:?}", n,s,b);

    eat_and_dump(n);
    eat_and_dump(s);
    eat_and_dump(b);
}