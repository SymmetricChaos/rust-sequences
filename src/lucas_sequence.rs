use num::BigInt;

/// Any recurrence of the form
/// a_x = p * a_{x-1} - q * a_{x-2}
pub struct LucasU {
    a: BigInt,
    b: BigInt,
    p: BigInt,
    q: BigInt,
}

impl LucasU {
    pub fn new(p: i64, q: i64) -> Self {
        Self {
            a: BigInt::from(0),
            b: BigInt::from(1),
            p: BigInt::from(p),
            q: BigInt::from(q),
        }
    }
}

impl Iterator for LucasU {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let pa = &self.p * &self.a;
        let qb = &self.q * &self.b;
        let t = pa - qb;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

pub struct LucasV {
    a: BigInt,
    b: BigInt,
    p: BigInt,
    q: BigInt,
}

impl LucasV {
    pub fn new(p: i64, q: i64) -> Self {
        Self {
            a: BigInt::from(2),
            b: BigInt::from(p),
            p: BigInt::from(p),
            q: BigInt::from(q),
        }
    }
}

impl Iterator for LucasV {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let pa = &self.p * &self.a;
        let qb = &self.q * &self.b;
        let t = pa - qb;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::print_a_few!(
    LucasU::new(1, -1), 0, 10; // Fibonacci
    LucasV::new(1, -1), 0, 10; // Lucas
    LucasU::new(2, -1), 0, 10; // Pell
    LucasV::new(2, -1), 0, 10; // Pell-Lucas
    LucasU::new(1, -2), 0, 10; // Jacobsthal
    LucasV::new(1, -2), 0, 10; // Jacobsthal-Lucas
);
