use std::marker::PhantomData;

use num::{BigInt, One, Zero};

/// The unit function in number theory. A single 1 followed by infinite 0s. It is the identity for the Dirichlet convolution.
pub struct UnitSequence<T> {
    n: bool,
    _phantom: PhantomData<T>,
}

impl<T: One + Zero> UnitSequence<T> {
    pub fn new() -> Self {
        Self {
            n: true,
            _phantom: PhantomData,
        }
    }
}

impl UnitSequence<BigInt> {
    pub fn new_big() -> Self {
        Self {
            n: true,
            _phantom: PhantomData,
        }
    }
}

impl<T: One + Zero> Iterator for UnitSequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n {
            self.n = false;
            Some(T::one())
        } else {
            Some(T::zero())
        }
    }
}

crate::check_sequences!(
    UnitSequence::<i32>::new(), [1, 0, 0, 0, 0];
    UnitSequence::new_big(), [1, 0, 0, 0, 0];
);
