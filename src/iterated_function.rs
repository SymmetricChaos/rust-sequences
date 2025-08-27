use num::BigInt;

/// Any recurrence of the form
/// a_x = f(a_{x-1}) * g(a_{x-2})
pub struct IteratedFunction {
    a: BigInt,
    f: Box<dyn Fn(&BigInt) -> BigInt>,
}

impl IteratedFunction {
    pub fn new<F>(a: i64, f: F) -> Self
    where
        F: Fn(&BigInt) -> BigInt + 'static,
    {
        Self {
            a: BigInt::from(a),
            f: Box::new(f),
        }
    }
}

impl Iterator for IteratedFunction {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        self.a = (self.f)(&self.a);
        Some(out)
    }
}

crate::print_a_few!(
    IteratedFunction::new(5, |x| x*x-(x+2)), 0, 6;
);
