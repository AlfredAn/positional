use crate::peano::{Successor, Zero};

pub trait Value {
    const VALUE: u64;
}

impl Value for Zero {
    const VALUE: u64 = 0;
}

impl<T> Value for Successor<T>
where
    T: Value,
{
    const VALUE: u64 = T::VALUE + 1;
}

const _: () = const {
    use positional_macro::peano as p;
    assert!(<p!(0)>::VALUE == 0);
    assert!(<p!(1)>::VALUE == 1);
    assert!(<p!(2)>::VALUE == 2);
    assert!(<p!(3)>::VALUE == 3);
    assert!(<p!(4)>::VALUE == 4);
    assert!(<p!(1000)>::VALUE == 1000);
};
