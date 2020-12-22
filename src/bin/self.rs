struct Test {
    consume: u32,
}

impl Test {
    fn consume(self) {
        println!("consume self:{} n:{} {}", self.consume, !3, 0x10000);
        for i in 2..5 {
            println!("i {}", i);
        }
    }
}

fn main() {
    let t = Test { consume: 1 };
    t.consume();
    // t.consume();
}
