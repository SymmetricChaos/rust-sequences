use crate::utils::divisibility::prime_factorization;

/// Integers that can be the length of the hypotenuse of a primitive Pythagorean triple.
/// 5, 13, 17, 25, 29, 37, 41, 53, 61, 65...
pub struct Hypotenuse {
    ctr: u64,
}

impl Hypotenuse {
    /// Only u64 output is currently supported.
    pub fn new() -> Self {
        Self { ctr: 1 }
    }
}

impl Iterator for Hypotenuse {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            self.ctr = self.ctr.checked_add(1)?;

            for factor in prime_factorization(self.ctr).iter().map(|x| x.0) {
                if (factor - 1) % 4 != 0 {
                    continue 'outer;
                }
            }
            break;
        }

        Some(self.ctr)
    }
}

/// Integers that cannot be the length of the hypotenuse of a primitive Pythagorean triple.
/// 1, 2, 3, 4, 6, 7, 8, 9, 11, 12...
pub struct Nonhypotenuse {
    ctr: u64,
}

impl Nonhypotenuse {
    /// Only u64 output is currently supported.
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Nonhypotenuse {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            self.ctr = self.ctr.checked_add(1)?;

            for factor in prime_factorization(self.ctr).iter().map(|x| x.0) {
                if (factor - 1) % 4 == 0 {
                    continue 'outer;
                }
            }
            break;
        }

        Some(self.ctr)
    }
}

crate::check_sequences!(
    Hypotenuse::new(), [5, 13, 17, 25, 29, 37, 41, 53, 61, 65, 73, 85, 89, 97, 101, 109, 113, 125, 137, 145];
    Nonhypotenuse::new(), [1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 14, 16, 18, 19, 21, 22, 23, 24, 27, 28];
);
