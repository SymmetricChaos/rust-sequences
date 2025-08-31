use num::BigInt;

pub struct PartialSums {
    sum: BigInt,
    iter: Box<dyn Iterator<Item = BigInt>>,
}

impl PartialSums {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = BigInt> + 'static,
    {
        Self {
            sum: BigInt::from(0),
            iter: Box::new(iter),
        }
    }
}

impl Iterator for PartialSums {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.sum += self.iter.next()?;
        Some(out)
    }
}

pub struct PartialDiffs {
    sum: BigInt,
    iter: Box<dyn Iterator<Item = BigInt>>,
}

impl PartialDiffs {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = BigInt> + 'static,
    {
        Self {
            sum: BigInt::from(0),
            iter: Box::new(iter),
        }
    }
}

impl Iterator for PartialDiffs {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.sum -= self.iter.next()?;
        Some(out)
    }
}

pub struct PartialProds {
    sum: BigInt,
    iter: Box<dyn Iterator<Item = BigInt>>,
}

impl PartialProds {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = BigInt> + 'static,
    {
        Self {
            sum: BigInt::from(0),
            iter: Box::new(iter),
        }
    }
}

impl Iterator for PartialProds {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.sum.clone();
        self.sum *= self.iter.next()?;
        Some(out)
    }
}
