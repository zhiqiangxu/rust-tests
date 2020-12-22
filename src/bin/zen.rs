use std::mem;

fn main() {
    let x = &tp();
    let a = "abc";
    println!("{:p}", a);
    let mut b = a;
    println!("a {:p} {:p}  b {:p} {:p}", &a, a, &b, b);

    let a = [1, 2, 3];
    let b = a;
    println!("{:p}{:?},", &a, b);
    println!("{}", x);

    let c: u128 = 245;
    println!("c {} sizeof<*mut u8> {}", c, mem::size_of::<*mut u8>());

    {
        let mut x = "Hello World".to_string();

        let mut_x = &mut x;

        drop(x);

        // println!("{}", x);

        let s1 = String::from("hello");
        println!("s1 {:p}", &s1);
        let s2 = s1;
        println!("s2 {:p}", &s2);
    }

    {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(&arr[1..], [2, 3, 4, 5]);
        assert_eq!(&arr, &[1, 2, 3, 4, 5]);
        let vec = vec![1, 2, 3];
        assert_eq!(vec[..], [1, 2, 3]);
        assert_eq!(&vec[..], &[1, 2, 3]);

        assert_eq!(.., std::ops::RangeFull);
        // std::collections::HashSet

        // assert_eq!(&vec[..], &[1, 2, 3, 4]);
        // assert_eq!(&[1, 2, 3], [1, 2, 3]);

        let boxed = Box::new(Test { x: 1, y: 2 });
        let unboxed = *boxed;
        println!("boxed {:?} unboxed {:?} {:p}", boxed, unboxed, &2);

        // let a = 0;
        // let a_pos = a.is_positive();

        let f = |u: usize| {
            println!("{}", u);
        };

        println!("size: {}", std::mem::size_of_val(&f));
    }
}

fn tp() -> i32 {
    1
}

fn t() {
    println!("t");
}

#[derive(Debug, Copy, Clone)]
struct Test {
    x: i32,
    y: i8,
}
