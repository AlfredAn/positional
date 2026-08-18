pub struct PeanoZero;

pub struct PeanoSucc<T>(std::marker::PhantomData<T>);

pub trait PeanoInt {}

impl PeanoInt for PeanoZero {}

impl<T> PeanoInt for PeanoSucc<T> where T: PeanoInt {}
