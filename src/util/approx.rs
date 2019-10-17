pub trait ApproxFrom<T> {
    fn approx_from(x: T) -> Self;
}

impl<T> ApproxFrom<T> for T {
    fn approx_from(x: T) -> Self {
        x
    }
}

impl ApproxFrom<usize> for f64 {
    fn approx_from(x: usize) -> f64 {
        x as f64
    }
}

pub trait ApproxInto<T> {
    fn approx(self) -> T;
}

impl<A,B> ApproxInto<A> for B
    where A: ApproxFrom<B>
{
    fn approx(self) -> A {
        A::approx_from(self)
    }
}
