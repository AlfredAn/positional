#[derive(Default)]
pub struct PeanoZero;

pub struct PeanoSucc<T>(std::marker::PhantomData<T>);

impl<T> Default for PeanoSucc<T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

pub trait PeanoInt {}

impl PeanoInt for PeanoZero {}

impl<T> PeanoInt for PeanoSucc<T> where T: PeanoInt {}
