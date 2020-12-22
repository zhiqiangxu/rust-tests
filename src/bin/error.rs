use std::io;
use anyhow;

fn main() {
    test().unwrap();
}


fn test() -> anyhow::Result<u8> {
    Err(std::io::Error::new(io::ErrorKind::NotFound,"abc"))?
}