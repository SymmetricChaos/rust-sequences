use num::{CheckedAdd, CheckedMul, CheckedSub, Integer, One, Zero, rational::Ratio};

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

/// Sequence of numerators of a sequence of ratios.
pub struct Numerators<T> {
    iter: Box<dyn Iterator<Item = Ratio<T>>>,
}

impl<T> Numerators<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = Ratio<T>> + 'static,
    {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<T: Clone> Iterator for Numerators<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.numer().clone())
    }
}

/// Sequence of denominators of a sequence of ratios.
pub struct Denominator<T> {
    iter: Box<dyn Iterator<Item = Ratio<T>>>,
}

impl<T> Denominator<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = Ratio<T>> + 'static,
    {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<T: Clone> Iterator for Denominator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.denom().clone())
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

/// Sequence of reciprocals of a sequence of ratios. Returns None whenever the numerator of the original sequence is zero.
pub struct Reciprocals<T> {
    iter: Box<dyn Iterator<Item = Ratio<T>>>,
}

impl<T> Reciprocals<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = Ratio<T>> + 'static,
    {
        Self {
            iter: Box::new(iter),
        }
    }
}

impl<T: Clone + Integer> Iterator for Reciprocals<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.iter.next()?;
        if n.numer().is_zero() {
            None
        } else {
            Some(n.recip())
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
