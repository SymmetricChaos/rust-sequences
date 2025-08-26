use num::BigInt;

/// Any recurrence of the form
/// a_x = p * a_{x-1} + q * a_{x-2}
pub struct LucasSequence {
    a: BigInt,
    b: BigInt,
    p: BigInt,
    q: BigInt,
}

impl LucasSequence {
    pub fn new(a: i64, b: i64, p: i64, q: i64) -> Self {
        Self {
            a: BigInt::from(a),
            b: BigInt::from(b),
            p: BigInt::from(p),
            q: BigInt::from(q),
        }
    }
}

impl Iterator for LucasSequence {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let pa = &self.p * &self.a;
        let qb = &self.q * &self.b;
        let t = pa + qb;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::print_a_few!(
    LucasSequence::new(0,  1,  1, 1), 0, 10; // Fibonacci sequence
    LucasSequence::new(0,  1,  1, 2), 0, 10; // Pell sequence
    LucasSequence::new(3,  2, -2, 3), 0, 10;
);
