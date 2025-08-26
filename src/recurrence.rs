use num::BigInt;

/// Any recurrence of the form
/// a_x = f(a_{x-1}) + g(a_{x-2})
pub struct Additive {
    a: BigInt,
    b: BigInt,
    p: Box<dyn Fn(&BigInt) -> BigInt>,
    q: Box<dyn Fn(&BigInt) -> BigInt>,
}

impl Additive {
    pub fn new<P, Q>(a: i64, b: i64, p: P, q: Q) -> Self
    where
        P: Fn(&BigInt) -> BigInt + 'static,
        Q: Fn(&BigInt) -> BigInt + 'static,
    {
        Self {
            a: BigInt::from(a),
            b: BigInt::from(b),
            p: Box::new(p),
            q: Box::new(q),
        }
    }
}

impl Iterator for Additive {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let pa = (self.p)(&self.a);
        let qb = (self.q)(&self.b);
        let t = pa + qb;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// Any recurrence of the form
/// a_x = f(a_{x-1}) * g(a_{x-2})
pub struct Multiplicative {
    a: BigInt,
    b: BigInt,
    p: Box<dyn Fn(&BigInt) -> BigInt>,
    q: Box<dyn Fn(&BigInt) -> BigInt>,
}

impl Multiplicative {
    pub fn new<P, Q>(a: i64, b: i64, p: P, q: Q) -> Self
    where
        P: Fn(&BigInt) -> BigInt + 'static,
        Q: Fn(&BigInt) -> BigInt + 'static,
    {
        Self {
            a: BigInt::from(a),
            b: BigInt::from(b),
            p: Box::new(p),
            q: Box::new(q),
        }
    }
}

impl Iterator for Multiplicative {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let pa = (self.p)(&self.a);
        let qb = (self.q)(&self.b);
        let t = pa * qb;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

crate::print_a_few!(
    Additive::new(0, 1, |x| x + 1, |x| x * -2), 0, 10;
    Multiplicative::new(0, 1, |x| x - 2, |x| x * -2), 0, 10;
);
