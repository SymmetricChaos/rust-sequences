use num::BigInt;

/// Any recurrence of the form
/// a_x = p * a_{x-1} + q * a_{x-2}
pub struct AdditiveLinear {
    a: BigInt,
    b: BigInt,
    p: BigInt,
    q: BigInt,
}

impl AdditiveLinear {
    pub fn new(a: i64, b: i64, p: i64, q: i64) -> Self {
        Self {
            a: BigInt::from(a),
            b: BigInt::from(b),
            p: BigInt::from(p),
            q: BigInt::from(q),
        }
    }
}

impl Iterator for AdditiveLinear {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = (&self.p * &self.a) + (&self.q * &self.b);
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

/// Any recurrence of the form
/// a_x = p * a_{x-1} * q * a_{x-2}
/// equivalently
/// a_x = m * a_{x-1} * a_{x-2}
/// where m = pq
pub struct MultiplicativeLinear {
    a: BigInt,
    b: BigInt,
    m: BigInt,
}

impl MultiplicativeLinear {
    /// Give p and q as for AdditiveLinear, simplified to a single multiplication
    pub fn new(a: i64, b: i64, p: i64, q: i64) -> Self {
        Self {
            a: BigInt::from(a),
            b: BigInt::from(b),
            m: BigInt::from(p) * BigInt::from(q),
        }
    }

    /// Give the constant m directly
    pub fn new_m(a: i64, b: i64, m: i64) -> Self {
        Self {
            a: BigInt::from(a),
            b: BigInt::from(b),
            m: BigInt::from(m),
        }
    }
}

impl Iterator for MultiplicativeLinear {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = &self.m * &self.a * &self.b;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

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
    AdditiveLinear::new(0, 1, 2, 3), 0, 10;
    MultiplicativeLinear::new(1, 2, 3, 4), 0, 10;
    Additive::new(0, 1, |x| x + 1, |x| x * -2), 0, 10;
    Multiplicative::new(0, 1, |x| x - 2, |x| x * -2), 0, 10;
);
