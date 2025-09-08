use num::{BigInt, One, Zero};

/// The integer partitions
pub struct Partition {
    values: Vec<BigInt>,
    ctr: usize,
}

impl Partition {
    pub fn new() -> Self {
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

crate::check_iteration_times!(
    Partition::new(), 27_000;
);

crate::check_sequences!(
    Partition::new(), 0, 10, [1, 1, 2, 3, 5, 7, 11, 15, 22, 30];
);
