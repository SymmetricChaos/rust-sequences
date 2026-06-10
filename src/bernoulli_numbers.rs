use crate::core::{alternating::Alternating, traits::Increment};
use num::{BigInt, CheckedAdd, CheckedMul, FromPrimitive, One, Zero, rational::Ratio};

#[cfg(feature = "big_int")]
/// The Bernoulli numbers.
///
/// ```text
/// 1, ±1/2, 1/6, 0, -1/30, 0, 1/42, 0, -1/30, 0, 5/66...
/// ```
pub struct Bernoulli<T> {
    m: usize,
    n: T,
}

#[cfg(feature = "big_int")]
impl Bernoulli<BigInt> {
    pub fn new_plus_big() -> Self {
        Self {
            m: 0,
            n: BigInt::one(),
        }
    }

    pub fn new_minus_big() -> Self {
        Self {
            m: 0,
            n: BigInt::zero(),
        }
    }
}

#[cfg(feature = "big_int")]
impl Iterator for Bernoulli<BigInt> {
    type Item = Ratio<BigInt>;

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

        self.m.incr()?;

        Some(Ratio::new(
            sum.numer().clone().try_into().ok()?,
            sum.denom().clone().try_into().ok()?,
        ))
    }
}

crate::check_sequences!(
    Bernoulli::new_plus_big(), ["1", "1/2", "1/6", "0", "-1/30", "0", "1/42", "0", "-1/30"];
);
