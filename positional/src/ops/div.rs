use crate::{
    ops::lt::{IsLt, NotLt},
    prelude::*,
};

pub trait Divide<T> {
    type Quotient;
    type Remainder;
}

pub type Quotient<T1, T2> = <T1 as Divide<T2>>::Quotient;
pub type Remainder<T1, T2> = <T1 as Divide<T2>>::Remainder;

impl<T1, T2> Divide<Successor<T2>> for T1
where
    T1: Lt<Successor<T2>>,
    (T1, Successor<T2>): Helper<<T1 as Lt<Successor<T2>>>::Lt>,
{
    type Quotient = <(T1, Successor<T2>) as Helper<<T1 as Lt<Successor<T2>>>::Lt>>::Quotient;
    type Remainder = <(T1, Successor<T2>) as Helper<<T1 as Lt<Successor<T2>>>::Lt>>::Remainder;
}

mod hidden {
    pub trait Helper<B> {
        type Quotient;
        type Remainder;
    }
}

use hidden::Helper;

impl<T1, T2> Helper<True> for (T1, T2)
where
    T1: IsLt<T2>,
{
    type Quotient = Zero;
    type Remainder = T1;
}

impl<T1, T2> Helper<False> for (T1, T2)
where
    T1: NotLt<T2>,
    T1: Subtract<T2>,
    Difference<T1, T2>: Divide<T2>,
{
    type Quotient = Successor<Quotient<Difference<T1, T2>, T2>>;
    type Remainder = Remainder<Difference<T1, T2>, T2>;
}

const _: () = const {
    use positional_macro::peano as p;

    assert!(equal::<Quotient<p!(0), p!(1)>, p!(0)>());
    assert!(equal::<Quotient<p!(0), p!(2)>, p!(0)>());
    assert!(equal::<Quotient<p!(1), p!(1)>, p!(1)>());
    assert!(equal::<Quotient<p!(1), p!(2)>, p!(0)>());
    assert!(equal::<Quotient<p!(2), p!(1)>, p!(2)>());
    assert!(equal::<Quotient<p!(2), p!(2)>, p!(1)>());
    assert!(equal::<Quotient<p!(3), p!(1)>, p!(3)>());
    assert!(equal::<Quotient<p!(3), p!(2)>, p!(1)>());
    assert!(equal::<Quotient<p!(4), p!(1)>, p!(4)>());
    assert!(equal::<Quotient<p!(4), p!(2)>, p!(2)>());

    assert!(equal::<Remainder<p!(0), p!(1)>, p!(0)>());
    assert!(equal::<Remainder<p!(0), p!(2)>, p!(0)>());
    assert!(equal::<Remainder<p!(1), p!(1)>, p!(0)>());
    assert!(equal::<Remainder<p!(1), p!(2)>, p!(1)>());
    assert!(equal::<Remainder<p!(2), p!(1)>, p!(0)>());
    assert!(equal::<Remainder<p!(2), p!(2)>, p!(0)>());
    assert!(equal::<Remainder<p!(3), p!(1)>, p!(0)>());
    assert!(equal::<Remainder<p!(3), p!(2)>, p!(1)>());
    assert!(equal::<Remainder<p!(4), p!(1)>, p!(0)>());
    assert!(equal::<Remainder<p!(4), p!(2)>, p!(0)>());
};
