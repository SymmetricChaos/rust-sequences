use crate::core::alternating::Alternating;
use num::{BigInt, CheckedAdd, CheckedMul, FromPrimitive, One, Signed, Zero, rational::Ratio};
use std::marker::PhantomData;

/// The Bernoulli numbers.
/// Either the plus or minus version of the sequence may be chosen.
///
/// 1, ±1/2, 1/6, 0, -1/30, 0, 1/42, 0, -1/30, 0, 5/66...
pub struct Bernoulli<T> {
    m: usize,
    phantom: PhantomData<T>,
    n: BigInt,
}

impl<T: Signed> Bernoulli<T>
where
    T: TryFrom<BigInt>,
{
    /// Internal calculations are done using BigInt and converted for output so there is no gain in speed or memory usage over ::new_big().
    pub fn new_plus() -> Self {
        Self {
            m: 0,
            phantom: PhantomData,
            n: BigInt::one(),
        }
    }

    /// Internal calculations are done using BigInt and converted for output so there is no gain in speed or memory usage over ::new_big().
    pub fn new_minus() -> Self {
        Self {
            m: 0,
            phantom: PhantomData,
            n: BigInt::zero(),
        }
    }
}

impl Bernoulli<BigInt> {
    pub fn new_plus_big() -> Self {
        Self::new_plus()
    }

    pub fn new_minus_big() -> Self {
        Self::new_minus()
    }
}

impl<T> Iterator for Bernoulli<T>
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
            for (j, s) in (0..=k).zip(Alternating::<BigInt>::pos_neg()) {
                let jb = BigInt::from_usize(j)?;
                j_sum = j_sum
                    + num::integer::binomial(kb.clone(), jb.clone())
                        * s
                        * (jb + &self.n).pow(self.m as u32);
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

crate::check_sequences!(
    Bernoulli::new_plus_big(), ["1", "1/2", "1/6", "0", "-1/30", "0", "1/42", "0", "-1/30"];
);
