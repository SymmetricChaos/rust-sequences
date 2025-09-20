use std::ops::Sub;

use num::{CheckedAdd, CheckedMul, CheckedSub, Integer, One, Signed, Zero, rational::Ratio};

/// Sequence of partial sums. Returns None if overflow occurs or sequence ends.
pub struct PartialSums<T> {
    sum: T,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T: Zero> PartialSums<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            sum: T::zero(),
            iter: Box::new(iter),
        }
    }
}

impl<T: CheckedAdd + Clone> Iterator for PartialSums<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.sum = self.sum.checked_add(&self.iter.next()?)?;
        Some(out)
    }
}

/// Sequence of alternating partial sums. Returns None if overflow occurs or sequence ends.
pub struct PartialSumsAlternating<T> {
    sum: T,
    iter: Box<dyn Iterator<Item = T>>,
    subtract: bool,
}

impl<T: Zero> PartialSumsAlternating<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            sum: T::zero(),
            iter: Box::new(iter),
            subtract: false,
        }
    }
}

impl<T: CheckedAdd + CheckedSub + Clone> Iterator for PartialSumsAlternating<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        if self.subtract {
            self.sum = self.sum.checked_add(&self.iter.next()?)?;
        } else {
            self.sum = self.sum.checked_sub(&self.iter.next()?)?;
        }
        self.subtract = !self.subtract;
        Some(out)
    }
}

/// Sequence of partial products. Returns None if overflow occurs or sequence ends.
pub struct PartialProds<T> {
    prod: T,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T: One> PartialProds<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            prod: T::one(),
            iter: Box::new(iter),
        }
    }
}

impl<T: CheckedMul + Clone> Iterator for PartialProds<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.prod.clone();
        self.prod = self.prod.checked_mul(&self.iter.next()?)?;
        Some(out)
    }
}

/// The absolute differences of every adjecent pair from a sequence.
pub struct AbsDiffs<T> {
    prev: T,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T> AbsDiffs<T> {
    pub fn new<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            prev: iter.next().unwrap(),
            iter: Box::new(iter),
        }
    }
}

impl<T: CheckedSub + Clone + Ord> Iterator for AbsDiffs<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.iter.next()?;
        let out = if self.prev > cur {
            self.prev.checked_sub(&cur)?
        } else {
            cur.checked_sub(&self.prev)?
        };
        self.prev = cur;
        Some(out)
    }
}

/// The first differences of a sequence: f(x+1) - f(x)
/// Requires signed values
pub struct Diffs<T> {
    prev: T,
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T: Signed> Diffs<T> {
    pub fn new<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            prev: iter.next().unwrap(),
            iter: Box::new(iter),
        }
    }
}

impl<T: Sub + Clone + Signed> Iterator for Diffs<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.iter.next()?;
        let out = cur.clone() - self.prev.clone();
        self.prev = cur;
        Some(out)
    }
}

/// The boustrophedon transform of a sequence
pub struct Boustrophedon<T> {
    iter: Box<dyn Iterator<Item = T>>,
    row: Vec<T>,
}

impl<T> Boustrophedon<T> {
    pub fn new<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            row: vec![iter.next().unwrap()],
            iter: Box::new(iter),
        }
    }
}

impl<T: Clone + CheckedAdd> Iterator for Boustrophedon<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.row.last().unwrap().clone();
        let k = self.row.len();
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(self.iter.next()?);
        for n in 1..=k {
            next_row.push(next_row[n - 1].checked_add(&self.row[k - n])?);
        }
        self.row = next_row;
        Some(out)
    }
}

/// The boustrophedon transform of a sequence
pub struct BoustrophedonTriangle<T> {
    iter: Box<dyn Iterator<Item = T>>,
    row: Vec<T>,
}

impl<T> BoustrophedonTriangle<T> {
    pub fn new<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            row: vec![iter.next().unwrap()],
            iter: Box::new(iter),
        }
    }
}

impl<T: Clone + CheckedAdd> Iterator for BoustrophedonTriangle<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = self.row.clone();
        let k = self.row.len();
        if k.is_even() {
            out.reverse();
        }
        let mut next_row = Vec::with_capacity(self.row.len() + 1);
        next_row.push(self.iter.next()?);
        for n in 1..=k {
            next_row.push(next_row[n - 1].checked_add(&self.row[k - n])?);
        }
        self.row = next_row;
        Some(out)
    }
}

/// Sequence of reciprocals of a sequence of integers. Returns None if the integer is zero.
pub struct IntegerReciprocals<T> {
    iter: Box<dyn Iterator<Item = T>>,
}

impl<T> IntegerReciprocals<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<T: Clone + Integer> Iterator for IntegerReciprocals<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.iter.next()?;
        if n.is_zero() {
            None
        } else {
            Some(Ratio::<T>::new(T::one(), n))
        }
    }
}

/// Given integer sequences N and D return the sequence n_i/d_i for each element of N and D.
/// Returns None if d_i is zero.
pub struct Ratios<T> {
    n: Box<dyn Iterator<Item = T>>,
    d: Box<dyn Iterator<Item = T>>,
}

impl<T> Ratios<T> {
    pub fn new<N, D>(n: N, d: D) -> Self
    where
        N: Iterator<Item = T> + 'static,
        D: Iterator<Item = T> + 'static,
    {
        Self {
            n: Box::new(n),
            d: Box::new(d),
        }
    }
}

impl<T: Clone + Integer> Iterator for Ratios<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let num = self.n.next()?;
        let den = self.d.next()?;
        if den.is_zero() {
            None
        } else {
            Some(Ratio::<T>::new(num, den))
        }
    }
}

/// The partial sums of Cesaro's summation method.
pub struct CesaroPartialSums<T> {
    sum: T,
    iter: Box<dyn Iterator<Item = T>>,
    ctr: T,
}

impl<T: One + Zero> CesaroPartialSums<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            sum: T::zero(),
            iter: Box::new(iter),
            ctr: T::zero(),
        }
    }
}

impl<T: Clone + CheckedAdd + Integer> Iterator for CesaroPartialSums<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr = self.ctr.checked_add(&T::one())?;
        self.sum = self.sum.checked_add(&self.iter.next()?)?;
        let out = Ratio::new(self.sum.clone(), self.ctr.clone());

        Some(out)
    }
}
