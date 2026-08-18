use crate::prelude::*;

pub trait Value {
    const VALUE: u64;
}

impl Value for PeanoZero {
    const VALUE: u64 = 0;
}

impl<T> Value for PeanoSucc<T>
where
    T: Value,
{
    const VALUE: u64 = T::VALUE + 1;
}

impl<R> Value for Term<R>
where
    R: Radix,
{
    const VALUE: u64 = 0;
}

impl<R, H, T> Value for Seq<R, H, T>
where
    Self: PosInt,
    R: Value,
    H: Value,
    T: Value,
{
    const VALUE: u64 = H::VALUE + R::VALUE * T::VALUE;
}

#[cfg(test)]
const _: () = const {
    use peano as p;
    assert!(<p!(0)>::VALUE == 0);
    assert!(<p!(1)>::VALUE == 1);
    assert!(<p!(2)>::VALUE == 2);
    assert!(<p!(3)>::VALUE == 3);
    assert!(<p!(4)>::VALUE == 4);
    assert!(<p!(1000)>::VALUE == 1000);
};
