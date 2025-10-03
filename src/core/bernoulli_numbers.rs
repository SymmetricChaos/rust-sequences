use std::marker::PhantomData;

use num::{BigInt, CheckedAdd, CheckedMul, FromPrimitive, One, Signed, Zero, rational::Ratio};

/// The Bernoulli numbers with -1/2 at index 1.
/// 1, -1/2, 1/6, 0, -1/30, 0, 1/42, 0, -1/30, 0, 5/66...
pub struct BernoulliMinus<T> {
    m: usize,
    phantom: PhantomData<T>,
}

impl<T: Signed> BernoulliMinus<T> {
    /// Internal calculations are still done using BigInt and converted for output so there is no gain in speed.
    pub fn new() -> Self {
        Self {
            m: 0,
            phantom: PhantomData,
        }
    }
}

impl BernoulliMinus<BigInt> {
    pub fn new_big() -> Self {
        Self {
            m: 0,
            phantom: PhantomData,
        }
    }
}

impl<T> Iterator for BernoulliMinus<T>
where
    T: TryFrom<BigInt>,
{
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = Ratio::zero();
        for k in 0..=self.m {
            let kb = BigInt::from_usize(k)?;
            let frac = Ratio::new(BigInt::one(), kb.clone() + BigInt::one());
            let mut j_sum = Ratio::zero();
            let mut sign = BigInt::one();
            for j in 0..=k {
                let jb = BigInt::from_usize(j)?;
                j_sum = j_sum
                    + num::integer::binomial(kb.clone(), jb.clone())
                        * sign.clone()
                        * jb.pow(self.m as u32);
                sign = -sign;
            }
            sum = sum.checked_add(&(frac.checked_mul(&j_sum)?))?;
        }
        self.m = self.m.checked_add(1)?;
        Some(Ratio::new_raw(
            sum.numer().clone().try_into().ok()?,
            sum.denom().clone().try_into().ok()?,
        ))
    }
}

/// The Bernoulli numbers with 1/2 at index 1.
/// 1, 1/2, 1/6, 0, -1/30, 0, 1/42, 0, -1/30, 0, 5/66...
pub struct BernoulliPlus<T> {
    m: usize,
    phantom: PhantomData<T>,
}

impl<T: Signed> BernoulliPlus<T> {
    /// Internal calculations are still done using BigInt and converted for output so there is no gain in speed.
    pub fn new() -> Self {
        Self {
            m: 0,
            phantom: PhantomData,
        }
    }
}

impl BernoulliPlus<BigInt> {
    pub fn new_big() -> Self {
        Self {
            m: 0,
            phantom: PhantomData,
        }
    }
}

impl<T> Iterator for BernoulliPlus<T>
where
    T: TryFrom<BigInt>,
{
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sum = Ratio::zero();
        for k in 0..=self.m {
            let kb = BigInt::from_usize(k)?;
            let frac = Ratio::new(BigInt::one(), kb.clone() + BigInt::one());
            let mut j_sum = Ratio::zero();
            let mut sign = BigInt::one();
            for j in 0..=k {
                let jb = BigInt::from_usize(j)?;
                j_sum = j_sum
                    + num::integer::binomial(kb.clone(), jb.clone())
                        * sign.clone()
                        * (jb + BigInt::one()).pow(self.m as u32);
                sign = -sign;
            }
            sum = sum.checked_add(&(frac.checked_mul(&j_sum)?))?;
        }
        self.m = self.m.checked_add(1)?;
        Some(Ratio::new_raw(
            sum.numer().clone().try_into().ok()?,
            sum.denom().clone().try_into().ok()?,
        ))
    }
}

crate::print_values!(
    BernoulliPlus::new_big(), 0, 20;
    BernoulliMinus::new_big(), 0, 20;
);
