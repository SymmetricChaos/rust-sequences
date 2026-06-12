use crate::Number;
use num::{
    BigInt, CheckedAdd, CheckedMul, CheckedSub, Integer, One, Zero, bigint::Sign, rational::Ratio,
};

/// The partial sums of the Wallis product, converging on pi.
///
/// ```text
/// 2, 8/3, 128/45, 512/175, 32768/11025, 131072/43659, 2097152/693693...
/// ```
pub struct WallisProduct<T> {
    ctr: T,
    product: Ratio<T>,
}

impl WallisProduct<Number> {
    pub fn new() -> Self {
        Self {
            ctr: 1,
            product: Ratio::one(),
        }
    }
}

#[cfg(feature = "big_int")]
impl WallisProduct<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::one(),
            product: Ratio::one(),
        }
    }
}

impl<T: Clone + Integer + CheckedMul + CheckedAdd> Iterator for WallisProduct<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.product.clone();
        let four = T::one() + T::one() + T::one() + T::one();
        let four_sq = four * self.ctr.clone() * self.ctr.clone();
        let term = Ratio::new(four_sq.clone(), four_sq.clone() - T::one());
        self.product = self.product.checked_mul(&term)?;
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out.checked_add(&out)?)
    }
}

/// The partial sums of the Madhava-Leibniz formula for pi. Four times the arctangent of one.
///
/// ```text
/// 0, 4, 8/3, 52/15, 304/105, 1052/315, 10312/3465, 147916/45045...
/// ```
pub struct Leibniz<T> {
    k: T,
    sign: Sign,
    sum: Ratio<T>,
}

impl Leibniz<Number> {
    pub fn new() -> Self {
        Self {
            k: 1,
            sign: Sign::Plus,
            sum: Ratio::new(0, 1),
        }
    }
}

#[cfg(feature = "big_int")]
impl Leibniz<BigInt> {
    pub fn new_big() -> Self {
        Self {
            k: BigInt::one(),
            sign: Sign::Plus,
            sum: Ratio::new(BigInt::zero(), BigInt::one()),
        }
    }
}

impl<T: Clone + CheckedAdd + Integer + CheckedMul + CheckedSub> Iterator for Leibniz<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Some(
            self.sum
                .clone()
                .checked_add(&self.sum)?
                .checked_add(&self.sum)?
                .checked_add(&self.sum)?,
        );
        let term = Ratio::new_raw(T::one(), self.k.clone());
        match self.sign {
            Sign::Minus => self.sum = self.sum.checked_sub(&term)?,
            Sign::Plus => self.sum = self.sum.checked_add(&term)?,
            Sign::NoSign => unreachable!("never occurs"),
        };
        self.sign = -self.sign;
        self.k = self.k.checked_add(&(T::one() + T::one()))?;
        out
    }
}

crate::sample_sequences!(
   Leibniz::new();
   WallisProduct::new();
);
