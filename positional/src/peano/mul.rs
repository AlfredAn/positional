use crate::peano::{
    Successor, Zero,
    add::{Add, Sum},
    eq::equal,
};

pub trait Multiply<T> {
    type Product;
}

pub type Product<T1, T2> = <T1 as Multiply<T2>>::Product;

impl Multiply<Zero> for Zero {
    type Product = Zero;
}

impl<T> Multiply<Successor<T>> for Zero {
    type Product = Zero;
}

impl<T> Multiply<Zero> for Successor<T> {
    type Product = Zero;
}

impl<T1, T2> Multiply<Successor<T2>> for Successor<T1>
where
    T1: Multiply<Successor<T2>>,
    Product<T1, Successor<T2>>: Add<Successor<T2>>,
{
    type Product = Sum<Product<T1, Successor<T2>>, Successor<T2>>;
}

const _: () = const {
    use positional_macro::peano as p;
    assert!(equal::<Product<p!(0), p!(0)>, p!(0)>());
    assert!(equal::<Product<p!(0), p!(1)>, p!(0)>());
    assert!(equal::<Product<p!(0), p!(2)>, p!(0)>());
    assert!(equal::<Product<p!(1), p!(0)>, p!(0)>());
    assert!(equal::<Product<p!(1), p!(1)>, p!(1)>());
    assert!(equal::<Product<p!(1), p!(2)>, p!(2)>());
    assert!(equal::<Product<p!(2), p!(0)>, p!(0)>());
    assert!(equal::<Product<p!(2), p!(1)>, p!(2)>());
    assert!(equal::<Product<p!(2), p!(2)>, p!(4)>());
};
