use crate::{Number, utils::exp_by_squaring::pow_mod};
use num::integer::gcd;

/// These primes are sufficient witnessses to do a deterministic Miller-Rabin test for all u64.
pub(super) const MR_WITNESSES_64: [Number; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum MRTest {
    Prime,
    Composite(Option<Number>),
}

impl std::fmt::Display for MRTest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MRTest::Prime => write!(f, "Prime"),
            MRTest::Composite(w) => match w {
                Some(d) => write!(f, "Composite({d})"),
                None => write!(f, "Composite"),
            },
        }
    }
}

/// A Miller-Rabin test that assumes the input is an odd number greater than 2. Returns either Prime or Composite. Composite may include a factor.
pub(super) fn miller_rabin(n: Number) -> MRTest {
    let mut d = (n - 1) / 2;
    let r = 1_i64 + d.trailing_zeros() as Number;
    d >>= d.trailing_zeros();

    'outer: for w in MR_WITNESSES_64.into_iter() {
        let mut x = pow_mod(w, d, n);

        if x == 1 || x == n - 1 {
            continue 'outer;
        }

        for _ in 0..r - 1 {
            let t = pow_mod(x, 2, n);

            if t == n - 1 {
                continue 'outer;
            }

            if t == 1 && x != 1 && x != (n - 1) {
                return MRTest::Composite(Some(gcd(x - 1, n)));
            }

            x = t;
        }
        return MRTest::Composite(None);
    }
    MRTest::Prime
}

/// Deterministic 64-bit primality test. First checks small prime factors then switches to Miller-Rabin.
pub fn is_prime(n: Number) -> bool {
    if n <= 1 {
        return false;
    }

    // Check by trial
    for witness in MR_WITNESSES_64 {
        if n == witness {
            return true;
        }
        if n % witness == 0 {
            return false;
        }
    }

    let mut d = (n - 1) / 2;
    let r = 1_i64 + d.trailing_zeros() as Number;
    d >>= d.trailing_zeros();

    'outer: for w in MR_WITNESSES_64.into_iter() {
        let mut x = pow_mod(w, d, n);

        if x == 1 || x == n - 1 {
            continue 'outer;
        }

        for _ in 0..r - 1 {
            let t = pow_mod(x, 2, n);

            if t == n - 1 {
                continue 'outer;
            }

            if t == 1 && x != 1 && x != (n - 1) {
                return false;
            }

            x = t;
        }
        return false;
    }
    true
}

/// Slightly faster primality check that assumes a number is not divisible by any witness and is not 0 or 1
pub(super) fn is_prime_partial(n: Number) -> bool {
    let mut d = (n - 1) / 2;
    let r = 1_i64 + d.trailing_zeros() as Number;
    d >>= d.trailing_zeros();

    'outer: for w in MR_WITNESSES_64.into_iter() {
        let mut x = pow_mod(w, d, n);

        if x == 1 || x == n - 1 {
            continue 'outer;
        }

        for _ in 0..r - 1 {
            let t = pow_mod(x, 2, n);

            if t == n - 1 {
                continue 'outer;
            }

            if t == 1 && x != 1 && x != (n - 1) {
                return false;
            }

            x = t;
        }
        return false;
    }
    true
}

#[cfg(test)]
#[test]
#[ignore = "visualization"]
fn factor_test() {
    println!("{:?}", miller_rabin(41));
    println!("{:?}", miller_rabin(5461));
    println!("{:?}", miller_rabin(1681));
    println!("{:?}", miller_rabin(4_104_071));
}
