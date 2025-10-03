use std::marker::PhantomData;

use num::{
    BigInt, CheckedAdd, CheckedMul, FromPrimitive, Integer, One, PrimInt, Signed, Zero,
    rational::Ratio,
};

pub struct BernoulliPlus<T> {
    m: usize,
    phantom: PhantomData<T>,
}

impl<T: PrimInt + Signed + Integer> BernoulliPlus<T> {
    pub fn new() -> Self {
        Self {
            m: 1,
            phantom: PhantomData,
        }
    }
}

impl BernoulliPlus<BigInt> {
    pub fn new_big() -> Self {
        Self {
            m: 1,
            phantom: PhantomData,
        }
    }
}

impl<T: Clone + Integer + FromPrimitive + CheckedAdd + CheckedMul + Signed> Iterator
    for BernoulliPlus<T>
{
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = Ratio::zero();
        for k in 0..=self.m {
            let frac = Ratio::new(T::one(), T::from_usize(k)? + T::one());
            let mut j_sum = Ratio::zero();
            let mut sign = T::one();
            for j in 0..=k {
                j_sum = j_sum
                    + T::from_usize(num::integer::binomial(k, j))?
                        * sign.clone()
                        * T::from_usize((j + 1).pow(self.m as u32))?;
                sign = -sign;
            }
            sum = sum.checked_add(&(frac * j_sum))?;
        }
        self.m += 1;
        Some(sum)
    }
}

crate::print_values!(
    BernoulliPlus::new_big(), 0, 10;
);
