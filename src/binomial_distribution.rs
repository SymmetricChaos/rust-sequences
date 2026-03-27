use num::{BigInt, One, Signed, integer::binomial, rational::Ratio};

/// The probability mass function for the binomial distribution given a positive integer number of trials and a positive rational probability of success.
pub struct BinomialDistribution {
    p: Ratio<BigInt>,
    q: Ratio<BigInt>,
    n: i32,
    k: i32,
}

impl BinomialDistribution {
    /// Panics if n or probability is not positive.
    pub fn new_big<T: Clone>(n: i32, probability: Ratio<T>) -> Self
    where
        BigInt: From<T>,
    {
        assert!(n.is_positive());
        let r = Ratio::new(
            BigInt::from(probability.numer().clone()),
            BigInt::from(probability.denom().clone()),
        );
        assert!(r.is_positive());
        assert!(r < Ratio::<BigInt>::one());
        Self {
            n,
            k: 0,
            p: r.clone(),
            q: Ratio::<BigInt>::one() - r,
        }
    }
}

impl Iterator for BinomialDistribution {
    type Item = Ratio<BigInt>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.k > self.n {
            return None;
        }

        let b = binomial(BigInt::from(self.n), BigInt::from(self.k));
        let p_succ = self.p.pow(self.k);
        let p_fail = self.q.pow(self.n - self.k);

        self.k = self.k.checked_add(1)?;

        Some(p_succ * p_fail * b)
    }
}

crate::print_sequences!(
    BinomialDistribution::new_big(15, Ratio::new(2, 3)).map(|x| crate::core::rational_digits::rational_decimal_string(x, 5).unwrap()), 15;
);
