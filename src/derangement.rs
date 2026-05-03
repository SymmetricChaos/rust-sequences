use num::{BigInt, CheckedAdd, CheckedMul, One, Zero};

/// The number of derangements for a set of n elements (starting from 0)
///
/// 1, 0, 1, 2, 9, 44, 265, 1854...
pub struct Derangements<T> {
    a: T,
    b: T,
    ctr: T,
    overflowed: bool,
}

impl<T: One + Zero> Derangements<T> {
    pub fn new() -> Self {
        Self {
            a: T::one(),
            b: T::zero(),
            ctr: T::one(),
            overflowed: false,
        }
    }
}

impl Derangements<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + CheckedAdd + CheckedMul + One> Iterator for Derangements<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.overflowed {
            return None;
        }

        let out = self.a.clone();

        let next = match self.ctr.checked_mul(&self.a.checked_add(&self.b)?) {
            Some(n) => n,
            None => {
                self.overflowed = true;
                return Some(out);
            }
        };
        self.a = self.b.clone();
        self.b = next;
        self.ctr = self.ctr.checked_add(&T::one())?;

        Some(out)
    }
}

crate::check_sequences!(
    Derangements::<u64>::new(), [1_u64, 0, 1, 2, 9, 44, 265, 1854, 14833, 133496, 1334961, 14684570, 176214841, 2290792932, 32071101049, 481066515734, 7697064251745, 130850092279664, 2355301661033953, 44750731559645106];
);
