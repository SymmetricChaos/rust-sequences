use num::{BigInt, CheckedAdd, CheckedMul, Integer, One, Zero, rational::Ratio};

/// Produce the convergents of a continued fraction given integer sequences representing the partial numerators and partial denominators.
pub struct ContinuedFraction<T> {
    a0: T,
    b0: T,
    a1: T,
    b1: T,
    nums: Box<dyn Iterator<Item = T>>,
    dens: Box<dyn Iterator<Item = T>>,
}

impl<T: Clone + Integer> ContinuedFraction<T> {
    pub fn new<N, D>(n: N, mut d: D) -> Self
    where
        N: Iterator<Item = T> + 'static,
        D: Iterator<Item = T> + 'static,
    {
        Self {
            a0: T::one(),
            b0: T::zero(),
            a1: d.next().unwrap(),
            b1: T::one(),
            nums: Box::new(n),
            dens: Box::new(d),
        }
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> Iterator for ContinuedFraction<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Ratio::new(self.a1.clone(), self.b1.clone());

        let n = self.nums.next()?;
        let d = self.dens.next()?;

        let a2 = d
            .checked_mul(&self.a1)?
            .checked_add(&n.checked_mul(&self.a0)?)?;
        let b2 = d
            .checked_mul(&self.b1)?
            .checked_add(&n.checked_mul(&self.b0)?)?;

        self.a0 = self.a1.clone();
        self.b0 = self.b1.clone();
        self.a1 = a2;
        self.b1 = b2;

        Some(out)
    }
}

/// Produce the convergents of a simple continued fraction given an integer sequence representing the partial denominators.
pub struct SimpleContinuedFraction<T> {
    a0: T,
    b0: T,
    a1: T,
    b1: T,
    dens: Box<dyn Iterator<Item = T>>,
}

impl<T: Clone + Integer> SimpleContinuedFraction<T> {
    pub fn new<I>(mut d: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            a0: T::one(),
            b0: T::zero(),
            a1: d.next().unwrap(),
            b1: T::one(),
            dens: Box::new(d),
        }
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedMul> Iterator for SimpleContinuedFraction<T> {
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Ratio::new(self.a1.clone(), self.b1.clone());

        let d = self.dens.next()?;

        let a2 = d.checked_mul(&self.a1)?.checked_add(&self.a0)?;
        let b2 = d.checked_mul(&self.b1)?.checked_add(&self.b0)?;

        self.a0 = self.a1.clone();
        self.b0 = self.b1.clone();
        self.a1 = a2;
        self.b1 = b2;

        Some(out)
    }
}

/// Produce the convergents of a simple continued fraction given an integer sequence representing the partial denominators.
pub struct SimplePeriodicContinuedFraction {
    a0: BigInt,
    b0: BigInt,
    a1: BigInt,
    b1: BigInt,
    dens: Box<dyn Iterator<Item = BigInt>>,
}

impl SimplePeriodicContinuedFraction {
    pub fn new(fixed: Vec<BigInt>, periodic: Vec<BigInt>) -> Self {
        Self {
            a0: BigInt::one(),
            b0: BigInt::zero(),
            a1: fixed[0].clone(),
            b1: BigInt::one(),
            dens: Box::new(
                fixed
                    .into_iter()
                    .chain(periodic.into_iter().cycle())
                    .skip(1),
            ),
        }
    }
}

impl Iterator for SimplePeriodicContinuedFraction {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = Ratio::new(self.a1.clone(), self.b1.clone());

        let d = self.dens.next()?;

        let a2 = d.checked_mul(&self.a1)?.checked_add(&self.a0)?;
        let b2 = d.checked_mul(&self.b1)?.checked_add(&self.b0)?;

        self.a0 = self.a1.clone();
        self.b0 = self.b1.clone();
        self.a1 = a2;
        self.b1 = b2;

        Some(out)
    }
}
