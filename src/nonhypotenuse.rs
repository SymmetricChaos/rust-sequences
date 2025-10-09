use crate::core::prime_factorization;
use std::marker::PhantomData;

pub struct Hypotenuse<T> {
    ctr: u64,
    _phantom: PhantomData<T>,
}

impl<T> Hypotenuse<T> {
    pub fn new() -> Self {
        Self {
            ctr: 1,
            _phantom: PhantomData,
        }
    }
}

impl<T> Iterator for Hypotenuse<T> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            self.ctr = self.ctr.checked_add(1)?;

            for factor in prime_factorization(self.ctr).iter().map(|x| x.0) {
                if (factor - 1) % 4 != 0 {
                    continue 'outer;
                }
            }
            break;
        }

        Some(self.ctr)
    }
}

pub struct Nonhypotenuse<T> {
    ctr: u64,
    _phantom: PhantomData<T>,
}

impl<T> Nonhypotenuse<T> {
    pub fn new() -> Self {
        Self {
            ctr: 0,
            _phantom: PhantomData,
        }
    }
}

impl<T> Iterator for Nonhypotenuse<T> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            self.ctr = self.ctr.checked_add(1)?;

            for factor in prime_factorization(self.ctr).iter().map(|x| x.0) {
                if (factor - 1) % 4 == 0 {
                    continue 'outer;
                }
            }
            break;
        }

        Some(self.ctr)
    }
}

crate::print_values!(
    Hypotenuse::<u32>::new(), 0, 20;
    Nonhypotenuse::<u32>::new(), 0, 20;
);
