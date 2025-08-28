use num::BigInt;

/// Any recurrence of the form
/// a_x = p * a_{x-1} - q * a_{x-2}
/// beginning with a_0 = 0 and a_1 = 1
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
        let t = (&self.p * &self.a) - (&self.q * &self.b);
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// Any recurrence of the form
/// a_x = p * a_{x-1} - q * a_{x-2}
/// beginning with a_0 = 2 and a_1 = p
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
        let t = (&self.p * &self.a) - (&self.q * &self.b);
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::check_sequences!(
    LucasU::new(1, -1), 0, 10, [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]; // Fibonacci
    LucasV::new(1, -1), 0, 10, [2, 1, 3, 4, 7, 11, 18, 29, 47, 76]; // Lucas
    LucasU::new(1, -2), 0, 10, [0, 1, 2, 5, 12, 29, 70, 169, 408, 985]; // Pell
);
