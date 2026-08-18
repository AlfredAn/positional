use crate::prelude::*;

pub trait Multiply<T> {
    type Product;
}

pub type Product<T1, T2> = <T1 as Multiply<T2>>::Product;

impl Multiply<PeanoZero> for PeanoZero {
    type Product = PeanoZero;
}

impl<T> Multiply<PeanoSucc<T>> for PeanoZero {
    type Product = PeanoZero;
}

impl<T> Multiply<PeanoZero> for PeanoSucc<T> {
    type Product = PeanoZero;
}

impl<T1, T2> Multiply<PeanoSucc<T2>> for PeanoSucc<T1>
where
    T1: Multiply<PeanoSucc<T2>>,
    Product<T1, PeanoSucc<T2>>: Add<PeanoSucc<T2>>,
{
    type Product = Sum<Product<T1, PeanoSucc<T2>>, PeanoSucc<T2>>;
}

#[cfg(test)]
const _: () = const {
    use peano as p;
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
