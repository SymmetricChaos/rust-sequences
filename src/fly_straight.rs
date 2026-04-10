use num::{BigInt, CheckedAdd, Integer};

/// Solane's "fly straight" sequence. Although it appears chaotic after the 638th term is suddenly falls into a simple pattern.
///
/// 1, 1, 4, 8, 2, 8, 4, 12, 3, 1, 12, 24, 2, 16...
pub struct FlyStraight<T> {
    a: T,
    b: T,
    ctr: T,
}

impl<T: CheckedAdd + Clone + Integer> FlyStraight<T> {
    pub fn new() -> Self {
        Self {
            a: T::one(),
            b: T::one(),
            ctr: T::one() + T::one(),
        }
    }
}

impl FlyStraight<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for FlyStraight<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();

        let g = self.ctr.gcd(&self.b);

        if g.is_one() {
            self.a = self.b.clone();
            self.b = self.a.checked_add(&self.ctr)?.checked_add(&T::one())?;
        } else {
            self.a = self.b.clone();
            self.b = self.a.clone() / g;
        }

        self.ctr = self.ctr.checked_add(&T::one())?;

        Some(out)
    }
}

/// A variation of the "fly straight" sequence that become periodic at the 82nd term.
///
/// 1, 4, 2, 7, 13, 20, 10, 19, 29, 40, 4, 17, 31...
pub struct A255140<T> {
    a: T,
    b: T,
    ctr: T,
}

impl<T: CheckedAdd + Clone + Integer> A255140<T> {
    pub fn new() -> Self {
        Self {
            a: T::one(),
            b: T::one() + T::one() + T::one() + T::one(),
            ctr: T::one() + T::one(),
        }
    }
}

impl A255140<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for A255140<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();

        let g = self.ctr.gcd(&self.b);

        if g.is_one() {
            self.a = self.b.clone();
            self.b = self
                .a
                .checked_add(&self.ctr)?
                .checked_add(&(T::one() + T::one()))?;
        } else {
            self.a = self.b.clone();
            self.b = self.a.clone() / g;
        }

        self.ctr = self.ctr.checked_add(&T::one())?;

        Some(out)
    }
}

crate::check_sequences!(
    FlyStraight::new_big(), [1, 1, 4, 8, 2, 8, 4, 12, 3, 1, 12, 24, 2, 16, 8, 24, 3, 21, 7, 27, 48, 16, 8, 32];
    A255140::new_big(), [1, 4, 2, 7, 13, 20, 10, 19, 29, 40, 4, 17, 31, 46, 23, 40, 5, 24, 4, 25, 5, 28];
);
