#![feature(const_fn)]

use phf::phf_map;

#[derive(Clone, Debug)]
pub enum Keyword {
    Loop,
    Continue,
    Break,
    Fn,
    Extern,
}

const KEYWORDS: phf::Map<&'static str, Keyword> = phf_map! {
    "loop" => Keyword::Loop,
    "continue" => Keyword::Continue,
    "break" => Keyword::Break,
    "fn" => Keyword::Fn,
    "extern" => Keyword::Extern,
};

fn parse_keyword(keyword: &str) -> Option<Keyword> {
    KEYWORDS.get(keyword).cloned()
}

fn main() {
    println!("{:?}", parse_keyword("loop").unwrap());
}
