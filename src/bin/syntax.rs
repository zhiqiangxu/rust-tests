// trait A {
//     type Input;
//     type Output;
// }

// struct MyFn<F>(F);

// impl<I,T,F> A for MyFn<F>
// where
//     F: Fn(I) -> T,
//     {
//         type Input = I;
//         type Output = T;
//     }

trait A<I> {
    type Output;
}

struct MyFn<F>(F);

impl<I,T,F> A<I> for MyFn<F>
where F:Fn(I)->T {
    type Output = T;
}

fn main() {

}