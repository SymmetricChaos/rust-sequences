use num::{BigInt, CheckedAdd, Integer, One, Zero, rational::Ratio};

pub trait Padic {
    fn padic_abs(&self, p: u32) -> Ratio<BigInt>;
    fn padic_val(&self, p: u32) -> BigInt;
}

impl Padic for BigInt {
    fn padic_abs(&self, p: u32) -> Ratio<BigInt> {
        if self.is_zero() {
            return Ratio::zero();
        }
        let mut denom = BigInt::one();
        let p = BigInt::from(p);
        let mut n = self.clone();
        loop {
            let (q, r) = n.div_rem(&p);
            n = q;
            if r.is_zero() {
                denom = denom * p.clone();
            } else {
                break;
            }
        }
        Ratio::new(BigInt::one(), denom)
    }

    fn padic_val(&self, p: u32) -> BigInt {
        let mut val = BigInt::zero();
        let mut n = self.clone();
        let p = BigInt::from(p);
        loop {
            let (q, r) = n.div_rem(&p);
            n = q;
            if r.is_zero() {
                val = val + BigInt::one();
            } else {
                break;
            }
        }
        val
    }
}

impl Padic for Ratio<BigInt> {
    fn padic_abs(&self, p: u32) -> Ratio<BigInt> {
        self.numer().padic_abs(p) * self.denom().padic_abs(p).recip()
    }

    fn padic_val(&self, p: u32) -> BigInt {
        self.numer().padic_val(p) - self.denom().padic_val(p)
    }
}

/// k-adic valuation of an integer
pub fn padic_valuation<T: CheckedAdd + Clone + Integer>(n: &T, p: &T) -> T {
    let mut val = T::zero();
    let mut n = n.clone();
    loop {
        let (q, r) = n.div_rem(&p);
        n = q;
        if r.is_zero() {
            val = val + T::one();
        } else {
            break;
        }
    }
    val
}

/// k-adic absolute value of an integer
pub fn padic_abs<T: CheckedAdd + Clone + Integer>(n: &T, p: &T) -> Ratio<T> {
    if n.is_zero() {
        return Ratio::zero();
    }
    let mut denom = T::one();
    let mut n = n.clone();
    loop {
        let (q, r) = n.div_rem(&p);
        n = q;
        if r.is_zero() {
            denom = denom * p.clone();
        } else {
            break;
        }
    }
    Ratio::new(T::one(), denom)
}
