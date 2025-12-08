use itertools::Itertools;
use num::{BigInt, CheckedAdd, CheckedSub, One, Zero};

/// The number of partitons for each integer.
pub struct Partition {
    values: Vec<BigInt>,
    ctr: usize,
}

impl Partition {
    pub fn new_big() -> Self {
        Self {
            values: vec![BigInt::one()],
            ctr: 0,
        }
    }
}

impl Iterator for Partition {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.values[self.ctr].clone();

        self.ctr += 1;
        let mut parts = BigInt::zero();
        let mut sign = 0;

        for p in crate::figurate::GeneralizedPentagonalGeneric::<i64>::new().skip(1) {
            let p = TryInto::<usize>::try_into(p).ok()?;
            if let Some(idx) = self.ctr.checked_sub(p) {
                if sign < 2 {
                    parts += &self.values[idx];
                } else {
                    parts -= &self.values[idx];
                }
                sign = (sign + 1) % 4;
            } else {
                break;
            }
        }

        self.values.push(parts.clone());

        Some(out)
    }
}

//    if n == 0:
//         yield []
//     if n <= 0:
//         return
//     for p in revlex_partitions(n-1):
//         if len(p) == 1 or (len(p) > 1 and p[-1] < p[-2]):
//             p[-1] += 1
//             yield p
//             p[-1] -= 1
//         p.append(1)
//         yield p
//         p.pop()

/// The partitions of a non-negative integer.
pub struct PartitionsN<T> {
    n: T,
    part: Vec<T>,
}

impl<T: Clone> PartitionsN<T> {
    pub fn new(n: T) -> Self {
        Self {
            n: n.clone(),
            part: vec![n],
        }
    }
}

impl PartitionsN<BigInt> {
    pub fn new_big<T>(n: T) -> Self
    where
        BigInt: From<T>,
    {
        let n = BigInt::from(n);
        Self {
            n: n.clone(),
            part: vec![n],
        }
    }
}

impl<T: CheckedSub + Clone + One> Iterator for PartitionsN<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.part.clone();

        for p in PartitionsN::new(self.n.checked_sub(&T::one())?) {}

        Some(out)
    }
}

/// The partitions of each non-negative integer.
pub struct Partitions<T> {
    ctr: T,
}

impl<T: Zero> Partitions<T> {
    pub fn new() -> Self {
        Self { ctr: T::zero() }
    }
}

impl Partitions<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + One> Iterator for Partitions<T> {
    type Item = Vec<Vec<T>>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = PartitionsN::<T>::new(self.ctr.clone()).collect_vec();
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

crate::check_iteration_times!(
    Partition::new_big(), 27_000;
);

crate::check_sequences!(
    Partition::new_big(), 0, 10, [1, 1, 2, 3, 5, 7, 11, 15, 22, 30];
);
