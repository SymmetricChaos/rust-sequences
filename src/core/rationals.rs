use num::{BigInt, CheckedAdd, CheckedSub, Integer, One, Signed, Zero, rational::Ratio};

/// The rational numbers in anti-diagonal order
pub struct Rationals<N> {
    numer: N,
    denom: N,
    row: N,
}

impl<N: CheckedAdd + CheckedSub + Clone + Ord + Integer> Rationals<N> {
    /// The non-negative rationals
    pub fn new() -> Self {
        Self {
            numer: N::one(),
            denom: N::one(),
            row: N::zero(),
        }
    }

    /// The positive rationals
    pub fn new_pos() -> Self {
        Self {
            numer: N::one(),
            denom: N::one(),
            row: N::one(),
        }
    }
}

impl Rationals<BigInt> {
    /// The non-negative rationals
    pub fn new_big() -> Self {
        Self {
            numer: BigInt::one(),
            denom: BigInt::one(),
            row: BigInt::zero(),
        }
    }

    /// The positive rationals
    pub fn new_big_pos() -> Self {
        Self {
            numer: BigInt::one(),
            denom: BigInt::one(),
            row: BigInt::one(),
        }
    }
}

impl<N: CheckedAdd + CheckedSub + Clone + Ord + Integer> Iterator for Rationals<N> {
    type Item = Ratio<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row.is_zero() {
            self.row = self.row.checked_add(&N::one())?;
            return Some(Ratio::zero());
        }

        // new_raw() justified because gcd is always checked before we get here
        let out = Ratio::new_raw(self.numer.clone(), self.denom.clone());

        loop {
            self.numer = self.numer.checked_sub(&N::one())?;
            self.denom = self.denom.checked_add(&N::one())?;
            if self.numer.is_zero() {
                self.row = self.row.checked_add(&N::one())?;
                self.numer = self.row.clone();
                self.denom = N::one();
            }
            if self.numer.gcd(&self.denom) == N::one() {
                break;
            }
        }

        Some(out)
    }
}

/// All rational numbers, starting from zero, with the positive rationals in anti-diagonal order each followed by its negative.
pub struct RationalsSigned<N> {
    numer: N,
    denom: N,
    row: N,
    positive: bool,
}

impl<N: CheckedAdd + CheckedSub + Clone + Ord + Integer + Signed> RationalsSigned<N> {
    pub fn new() -> Self {
        Self {
            numer: N::one(),
            denom: N::one(),
            row: N::zero(),
            positive: true,
        }
    }
}

impl RationalsSigned<BigInt> {
    pub fn new_big() -> Self {
        Self {
            numer: BigInt::one(),
            denom: BigInt::one(),
            row: BigInt::zero(),
            positive: true,
        }
    }
}

impl<N: CheckedAdd + CheckedSub + Clone + Ord + Integer + Signed> Iterator for RationalsSigned<N> {
    type Item = Ratio<N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row.is_zero() {
            self.row = self.row.checked_add(&N::one())?;
            return Some(Ratio::zero());
        }

        if !self.positive {
            self.positive = true;
            // new_raw() justified because gcd is always checked before we get here
            let out = Some(Ratio::new_raw(-self.numer.clone(), self.denom.clone()));
            loop {
                self.numer = self.numer.checked_sub(&N::one())?;
                self.denom = self.denom.checked_add(&N::one())?;
                if self.numer.is_zero() {
                    self.row = self.row.checked_add(&N::one())?;
                    self.numer = self.row.clone();
                    self.denom = N::one();
                }
                if self.numer.gcd(&self.denom) == N::one() {
                    return out;
                }
            }
        } else {
            self.positive = false;
            return Some(Ratio::new_raw(self.numer.clone(), self.denom.clone()));
        };
    }
}

crate::print_values!(
    Rationals::<u32>::new(), 0, 20;
    RationalsSigned::<i32>::new(), 0, 20;
);
