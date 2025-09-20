use num::{BigInt, PrimInt};

/// The Pell numbers.
/// 0, 1, 2, 5, 12, 29, 70, 169, 408, 985...
pub struct Pell {
    a: BigInt,
    b: BigInt,
}

impl Pell {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::from(0),
            b: BigInt::from(1),
        }
    }
}

impl Iterator for Pell {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = &self.a + (&self.b * 2);
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// The Pell numbers.
/// 0, 1, 2, 5, 12, 29, 70, 169, 408, 985...
pub struct PellGeneric<T> {
    a: T,
    b: T,
}

impl<T: PrimInt> PellGeneric<T> {
    pub fn new_big() -> Self {
        Self {
            a: T::zero(),
            b: T::one(),
        }
    }
}

impl<T: PrimInt> Iterator for PellGeneric<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let two = T::one() + T::one();
        let t = self.a.checked_add(&(self.b.checked_mul(&two)?))?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// The companion Pell numbers.
/// 2, 2, 6, 14, 34, 82, 198, 478, 1154, 2786...
pub struct CompanionPell {
    a: BigInt,
    b: BigInt,
}

impl CompanionPell {
    pub fn new_big() -> Self {
        Self {
            a: BigInt::from(2),
            b: BigInt::from(2),
        }
    }
}

impl Iterator for CompanionPell {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = &self.a + (&self.b * 2);
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// The companion Pell numbers.
/// 2, 2, 6, 14, 34, 82, 198, 478, 1154, 2786...
pub struct CompanionPellGeneric<T> {
    a: T,
    b: T,
}

impl<T: PrimInt> CompanionPellGeneric<T> {
    pub fn new_big() -> Self {
        Self {
            a: T::one() + T::one(),
            b: T::one() + T::one(),
        }
    }
}

impl<T: PrimInt> Iterator for CompanionPellGeneric<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let two = T::one() + T::one();
        let t = self.a.checked_add(&(self.b.checked_mul(&two)?))?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_iteration_times!(
    Pell::new_big(), 90_000;
);

crate::check_sequences!(
    Pell::new_big(), 0, 10, [0, 1, 2, 5, 12, 29, 70, 169, 408, 985];
    // PellGeneric::<u64>::new(), 0, 10, [0, 1, 2, 5, 12, 29, 70, 169, 408, 985];
    CompanionPell::new_big(), 0, 10, [2, 2, 6, 14, 34, 82, 198, 478, 1154, 2786];
    // CompanionPellGeneric::<u64>::new(), 0, 10, [2, 2, 6, 14, 34, 82, 198, 478, 1154, 2786];
);
