use crate::peano::{
    Four, One, Successor, Two, Zero,
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
    assert!(equal::<Product<Zero, Zero>, Zero>());
    assert!(equal::<Product<Zero, One>, Zero>());
    assert!(equal::<Product<Zero, Two>, Zero>());
    assert!(equal::<Product<One, Zero>, Zero>());
    assert!(equal::<Product<One, One>, One>());
    assert!(equal::<Product<One, Two>, Two>());
    assert!(equal::<Product<Two, Zero>, Zero>());
    assert!(equal::<Product<Two, One>, Two>());
    assert!(equal::<Product<Two, Two>, Four>());
};
