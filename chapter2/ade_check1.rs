
fn main() {
    let mut s= "world";
    {
        let mut changer = || s = "nope";
        changer()
    }
    assert_eq!(s, "world");
}