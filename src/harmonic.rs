use crate::{Number, core::traits::Increment};
use num::{
    BigInt, CheckedAdd, CheckedMul, CheckedSub, Integer, One, Zero, bigint::Sign, rational::Ratio,
};

/// The harmonic numbers, partial sums of the harmonic series. This sequence diverges.
///
/// ```text
/// 1, 3/2, 11/6, 25/12, 137/60, 49/20, 363/140, 761/280, 7129/2520...
/// ```
pub struct Harmonic<T> {
    ctr: T,
    sum: Ratio<T>,
}

impl Harmonic<Number> {
    pub fn new() -> Self {
        Self {
            ctr: 0,
            sum: Ratio::zero(),
        }
    }

    pub fn numers() -> impl Iterator<Item = Number> {
        Self::new().map(|q| q.numer().clone())
    }

    pub fn denoms() -> impl Iterator<Item = Number> {
        Self::new().map(|q| q.denom().clone())
    }
}

impl Harmonic<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
            sum: Ratio::zero(),
        }
    }

    pub fn numers_big() -> impl Iterator<Item = BigInt> {
        Self::new_big().map(|q| q.numer().clone())
    }

    pub fn denoms_big() -> impl Iterator<Item = BigInt> {
        Self::new_big().map(|q| q.denom().clone())
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedSub + Integer> Iterator for Harmonic<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        self.sum = self
            .sum
            .checked_add(&Ratio::new(T::one(), self.ctr.clone()))?;
        Some(self.sum.clone())
    }
}

/// The partial sums of the alternating harmonic series. They converge on the natural logarithm of 2.
///
/// ```text
/// 1/2, 1/6, 5/12, 13/60, 23/60, 101/420, 307/840, 641/2520, 893/2520...
/// ```
pub struct AlternatingHarmonic<T> {
    ctr: T,
    sum: Ratio<T>,
    sign: Sign,
}

impl AlternatingHarmonic<Number> {
    pub fn new() -> Self {
        Self {
            ctr: 1,
            sum: Ratio::zero(),
            sign: Sign::Plus,
        }
    }

    pub fn numers() -> impl Iterator<Item = Number> {
        Self::new().map(|q| q.numer().clone())
    }

    pub fn denoms() -> impl Iterator<Item = Number> {
        Self::new().map(|q| q.denom().clone())
    }
}

impl AlternatingHarmonic<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::one(),
            sum: Ratio::zero(),
            sign: Sign::Plus,
        }
    }

    pub fn numers_big() -> impl Iterator<Item = BigInt> {
        Self::new_big().map(|q| q.numer().clone())
    }

    pub fn denoms_big() -> impl Iterator<Item = BigInt> {
        Self::new_big().map(|q| q.denom().clone())
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + CheckedSub + Integer> Iterator
    for AlternatingHarmonic<T>
{
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        let q = Ratio::new(T::one(), self.ctr.clone());
        match self.sign {
            Sign::Minus => self.sum = self.sum.checked_sub(&q)?,
            Sign::NoSign => unreachable!("NoSign is never used"),
            Sign::Plus => self.sum = self.sum.checked_add(&q)?,
        }
        self.sign = -self.sign;
        Some(self.sum.clone())
    }
}

#[cfg(test)]
use crate::core::traits::DigitSequence;
crate::print_sequences!(
    AlternatingHarmonic::new_big().map(|q| q.digits(15).unwrap()), skip 100, 5; // Converges on ln(2) = 0.69314718056...
);

crate::check_sequences!(
    Harmonic::numers(), [1_i64, 3, 11, 25, 137, 49, 363, 761, 7129, 7381, 83711, 86021, 1145993, 1171733, 1195757, 2436559, 42142223, 14274301, 275295799, 55835135, 18858053, 19093197, 444316699, 1347822955, 34052522467, 34395742267, 312536252003, 315404588903, 9227046511387];
    Harmonic::denoms(), [1_i64, 2, 6, 12, 60, 20, 140, 280, 2520, 2520, 27720, 27720, 360360, 360360, 360360, 720720, 12252240, 4084080, 77597520, 15519504, 5173168, 5173168, 118982864, 356948592, 8923714800, 8923714800, 80313433200, 80313433200, 2329089562800, 2329089562800, 72201776446800];
    AlternatingHarmonic::numers(), [1_i64, 1, 5, 7, 47, 37, 319, 533, 1879, 1627, 20417, 18107, 263111, 237371, 52279, 95549, 1768477, 1632341, 33464927, 155685007, 166770367, 156188887, 3825136961, 3602044091, 19081066231, 18051406831, 57128792093, 7751493599, 236266661971];
    AlternatingHarmonic::denoms(), [1_i64, 2, 6, 12, 60, 60, 420, 840, 2520, 2520, 27720, 27720, 360360, 360360, 72072, 144144, 2450448, 2450448, 46558512, 232792560, 232792560, 232792560, 5354228880, 5354228880, 26771144400, 26771144400, 80313433200, 11473347600];
);

crate::sample_sequences!(
    Harmonic::new();
    AlternatingHarmonic::new();
);
