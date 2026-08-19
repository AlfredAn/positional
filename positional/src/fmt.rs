use std::{
    fmt::{Debug, Formatter, Result},
    marker::PhantomData,
};

use crate::prelude::*;

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn debug<T>()
where
    T: Default + Debug,
{
    println!("{:?}", T::default());
}

impl Debug for PeanoZero {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_tuple("Peano").field(&0).finish()
    }
}

impl<T> Debug for PeanoSucc<T>
where
    T: Value,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_tuple("Peano").field(&Self::VALUE).finish()
    }
}

struct DebugInner<T>(PhantomData<T>);

impl<T> Default for DebugInner<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<R> Debug for DebugInner<Term<R>>
where
    Term<R>: PosInt,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Term").finish()
    }
}

impl<R, H, T> Debug for DebugInner<Seq<R, H, T>>
where
    Seq<R, H, T>: PosInt,
    H: Value,
    DebugInner<T>: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_tuple("Seq")
            .field(&H::VALUE)
            .field(&DebugInner::<T>::default())
            .finish()
    }
}

impl<R> Debug for Term<R>
where
    Self: PosInt,
    R: Value,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_tuple("Term").field(&R::VALUE).finish()
    }
}

impl<R, H, T> Debug for Seq<R, H, T>
where
    Self: PosInt,
    R: Value,
    H: Value,
    DebugInner<T>: Debug,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_tuple("Seq")
            .field(&R::VALUE)
            .field(&H::VALUE)
            .field(&DebugInner::<T>::default())
            .finish()
    }
}
