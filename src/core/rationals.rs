use num::{BigInt, CheckedAdd, CheckedSub, Integer, One, Zero, rational::Ratio};

// The positive rational numbers in anti-diagonal order
pub struct Rationals<N> {
    numer: N,
    denom: N,
    row: N,
}

impl<N: One + Zero> Rationals<N> {
    pub fn new() -> Self {
        Self {
            numer: N::one(),
            denom: N::one(),
            row: N::one(),
        }
    }
}

impl Rationals<BigInt> {
    pub fn new_big() -> Self {
        Self {
            numer: BigInt::one(),
            denom: BigInt::one(),
            row: BigInt::one(),
        }
    }
}

impl<N: CheckedAdd + CheckedSub + Clone + One + Ord + Integer> Iterator for Rationals<N> {
    type Item = Ratio<N>;

    fn next(&mut self) -> Option<Self::Item> {
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

crate::print_values!(
    Rationals::<u32>::new(), 0, 20;
);
