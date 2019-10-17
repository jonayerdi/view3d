/// ApproxFrom trait and impl for Self
pub trait ApproxFrom<T> {
    fn approx_from(x: T) -> Self;
}

impl<T> ApproxFrom<T> for T {
    fn approx_from(x: T) -> Self {
        x
    }
}

/// ApproxFrom impls
impl ApproxFrom<usize> for f64 {
    fn approx_from(x: usize) -> f64 {
        x as f64
    }
}

/// ApproxInto impls are automatically generated from ApproxFrom impls
pub trait ApproxInto<T> {
    fn approx(self) -> T;
}

impl<A, B> ApproxInto<A> for B
where
    A: ApproxFrom<B>,
{
    fn approx(self) -> A {
        A::approx_from(self)
    }
}
