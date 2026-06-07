use crate::{Number, utils::divisibility::prime_factorization};

/// Hypotenuse Numbers. Integers that can be the length of the hypotenuse of a primitive right triangle with integer sides.
///
/// ```text
/// 5, 13, 17, 25, 29, 37, 41, 53, 61, 65, 73, 85, 89, 97, 101, 109...
/// ```
pub struct Hypotenuse {
    ctr: Number,
}

impl Hypotenuse {
    pub fn new() -> Self {
        Self { ctr: 1 }
    }
}

impl Iterator for Hypotenuse {
    type Item = Number;

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

/// Non-Hypotenuse Numbers. Positive integers that cannot be the length of the hypotenuse of any right triangle with integer sides. Not the complement of the hypotenuse numbers.
///
/// ```text
/// 1, 2, 3, 4, 6, 7, 8, 9, 11, 12, 14, 16, 18, 19, 21, 22, 23, 24, 27...
/// ```
pub struct Nonhypotenuse {
    ctr: Number,
}

impl Nonhypotenuse {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Nonhypotenuse {
    type Item = Number;

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

crate::sample_sequences!(
    Hypotenuse::new();
    Nonhypotenuse::new();
);
