trait T {
    fn hi(&self) {
        println!("hi")
    }
    fn f(&self) -> bool
    where
        Self: Copy,
    {
        return true;
    }
}
use async_trait::async_trait;
#[derive(Copy, Clone)]
struct t {
    a: i8,
}

#[async_trait]
pub trait Operator<M>: Send + Sync {
    type Functor;
    async fn process(&mut self);

    fn to_functor(&self) -> Option<Self::Functor>;
    fn monad_points(&self) -> Option<Vec<String>>;
    fn monad(&mut self, functor: M, mp: &str);
}

pub struct T1;

#[async_trait]
impl<M> Operator<M> for T1
where
    M: Copy + 'static,
{
    type Functor = u8;
    async fn process(&mut self) {
        return;
    }

    fn to_functor(&self) -> Option<Self::Functor> {
        return None;
    }
    fn monad_points(&self) -> Option<Vec<String>> {
        return None;
    }
    fn monad(&mut self, functor: M, mp: &str) {}
}

impl T for t {}
fn main() {
    let mut t1 = T1;
    t1.monad("str", "mp");

    let a = t { a: 0 };
    a.hi();
    a.f();

    let a = "hello";

    let mut b: &str;

    b = &a;

    println!("b {} ptr {:p} a ptr {:p}", b, b, a)
}
