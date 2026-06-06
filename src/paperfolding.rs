use crate::{Number, odd_part::OddPart};
use num::{BigInt, Integer, One, Zero};

/// The regular paperfolding sequence, derived from the direction of folds that ooccur when a sheet of paper is folded in half in the same direction repeatedly. Each term is 1 if the odd part of n is equal to 1 modulo 4 and otherwise is 0.
///
/// ```text
/// 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0...
/// ```
pub struct RegularPaperfolding<T> {
    odd_part: OddPart<T>,
}

impl RegularPaperfolding<Number> {
    pub fn new() -> Self {
        Self {
            odd_part: OddPart::new(),
        }
    }
}

impl RegularPaperfolding<BigInt> {
    pub fn new_big() -> Self {
        Self {
            odd_part: OddPart::new_big(),
        }
    }
}

impl Iterator for RegularPaperfolding<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.odd_part.next()?;

        if (n % 4) == 1 { Some(1) } else { Some(0) }
    }
}

impl Iterator for RegularPaperfolding<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.odd_part.next()?;

        if (n.mod_floor(&BigInt::from(4))).is_one() {
            Some(BigInt::one())
        } else {
            Some(BigInt::zero())
        }
    }
}

/// A general paperfolding sequence.
pub struct Paperfolding<T> {
    odd_part: OddPart<T>,
    iter: Box<dyn Iterator<Item = T>>,
}

impl Paperfolding<Number> {
    /// The iterator should output only 0s and 1s representing the directions of the folding. A iterator that gives all 1s produces the regular paperfolding sequence.
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = Number> + 'static,
    {
        Self {
            odd_part: OddPart::new(),
            iter: Box::new(iter),
        }
    }
}

impl Paperfolding<BigInt> {
    /// The iterator should output only 0s and 1s, representing the directions of the folding. A iterator that gives all 1s produces the regular paperfolding sequence.
    pub fn new_big<I>(iter: I) -> Self
    where
        I: Iterator<Item = BigInt> + 'static,
    {
        Self {
            odd_part: OddPart::new_big(),
            iter: Box::new(iter),
        }
    }
}

impl Iterator for Paperfolding<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.odd_part.next()?;

        if (n % 4) == 1 {
            self.iter.next()
        } else {
            match self.iter.next() {
                Some(b) => Some(1 - b),
                None => None,
            }
        }
    }
}

impl Iterator for Paperfolding<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.odd_part.next()?;

        if (n.mod_floor(&BigInt::from(4))).is_one() {
            self.iter.next()
        } else {
            Some(1 - self.iter.next()?)
        }
    }
}

crate::check_sequences!(
    RegularPaperfolding::new(), [1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0];
    Paperfolding::new([1, 1, 1, 1, 1, 1, 1].into_iter()), [1, 1, 0, 1, 1, 0, 0];
    Paperfolding::new([0, 0, 0, 0, 0, 0, 0].into_iter()), [0, 0, 1, 0, 0, 1, 1];
    Paperfolding::new([0, 1, 0, 1, 0, 1, 0].into_iter()), [0, 1, 1, 1, 0, 0, 1];
);

crate::sample_sequences!(
    RegularPaperfolding::new();
);
