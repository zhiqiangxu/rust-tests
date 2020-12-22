fn main() {
    let n = (0..)
        .map(|n| n * n)
        .take_while(|n| *n < 100)
        .filter(|n| n % 2 == 0)
        .fold(0, |s, i| s + i);
    println!("result {}", n)
}
