use num::BigInt;

/// Any simple multiplicative recurrence
/// a_x = a_(x-1) * a_(x-2)
pub struct MultiplicativeRecurrence {
    a: BigInt,
    b: BigInt,
}

impl MultiplicativeRecurrence {
    pub fn new(a: i32, b: i32) -> Self {
        Self {
            a: BigInt::from(a),
            b: BigInt::from(b),
        }
    }
}

impl Iterator for MultiplicativeRecurrence {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_mul(&self.b)?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::print_a_few!(
    super::MultiplicativeRecurrence::new(1,2), 0, 10;
    super::MultiplicativeRecurrence::new(2,-3), 0, 10;
);
