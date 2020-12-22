fn main() {
    let fn_name = "anno";
    let mut b = String::from("hello");
    let push_data = move || {
        b.push_str("world !");
        b
    };

    println!("{}", push_data())
}
