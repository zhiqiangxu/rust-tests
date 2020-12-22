use std::fs;

type A = u8;

fn alignment(size: u32, align: u32) -> u32 {
    let x = size | align;
    println!("x={}", x);
    x & x.wrapping_neg()
}
fn main() {
    let fns = fs::read_dir("../")
        .unwrap()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>()
        .join(";");

    println!("{} alignment({},{}) = {}", r#""""#, 34, 4, alignment(34, 4));
    println!("xxxxxxx {:?} {} {}", fns, 1e6, 10_i32.pow(6));
}
