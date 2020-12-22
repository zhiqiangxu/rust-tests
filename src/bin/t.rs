fn main() {
    let a = "hello xx ";

    let b: &str;

    b = &a;
    println!("b {} ptr {:p} a ptr {:p} {}", b, b, a, b == a);
}
