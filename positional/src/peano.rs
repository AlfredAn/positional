use crate::Zero;

pub struct Successor<T>(std::marker::PhantomData<T>);

pub trait PeanoInt {}

impl PeanoInt for Zero {}

impl<T> PeanoInt for Successor<T> where T: PeanoInt {}
