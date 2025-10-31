use num::{BigInt, One, Zero};

pub struct DirichletConvolution {
    ctr: usize,
    f: Box<dyn Fn(BigInt) -> BigInt>,
    g: Box<dyn Fn(BigInt) -> BigInt>,
}

impl DirichletConvolution {
    pub fn new_big<F, G>(f: F, g: G) -> Self
    where
        F: Fn(BigInt) -> BigInt + 'static,
        G: Fn(BigInt) -> BigInt + 'static,
    {
        Self {
            ctr: usize::one(),
            f: Box::new(f),
            g: Box::new(g),
        }
    }
}

impl Iterator for DirichletConvolution {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = BigInt::zero();
        for i in 1..(self.ctr + 1) {
            if (self.ctr % i).is_zero() {
                out += (self.f)(BigInt::from(i)) * (self.g)(BigInt::from(self.ctr / i))
            }
        }
        self.ctr = self.ctr.checked_add(1)?;

        Some(out)
    }
}
