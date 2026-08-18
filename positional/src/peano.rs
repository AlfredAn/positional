pub struct PeanoZero;

pub struct Successor<T>(std::marker::PhantomData<T>);

pub trait PeanoInt {}

impl PeanoInt for PeanoZero {}

impl<T> PeanoInt for Successor<T> where T: PeanoInt {}
