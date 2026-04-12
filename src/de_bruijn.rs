use crate::utils::factorial::factorial;
use num::{BigInt, CheckedMul, Integer};

/// The number of de Bruijn sequences for an alphabet with k symbols for each substring length n, starting at zero.
pub struct DeBruijn<T> {
    k: T,
    f: T,
    dividend: T,
    divisior: T,
    overflowed: bool,
}

impl<T: CheckedMul + Clone + Integer> DeBruijn<T> {
    /// All fixed width integers will overflow quickly due to a doubly exponential term.
    pub fn new(k: T) -> Self {
        let f = factorial(k.clone());
        Self {
            k,
            f,
            dividend: T::one(),
            divisior: T::one(),
            overflowed: false,
        }
    }
}

impl DeBruijn<BigInt> {
    pub fn new_big<G>(k: G) -> Self
    where
        BigInt: From<G>,
    {
        Self::new(BigInt::from(k))
    }
}

impl<T: CheckedMul + Clone + Integer> Iterator for DeBruijn<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.overflowed {
            return None;
        }
        let out = self.dividend.clone() / self.divisior.clone();

        // Advance dividend
        let mut ex = self.divisior.clone();
        while !ex.is_zero() {
            self.dividend = match self.dividend.checked_mul(&self.f) {
                Some(n) => n,
                None => {
                    self.overflowed = true;
                    return Some(out);
                }
            };
            ex = ex - T::one();
        }

        // Advance the divisor
        self.divisior = self.divisior.checked_mul(&self.k)?;
        Some(out)
    }
}

// TODO: how to iterate with this?
fn debruijn(t: usize, p: usize, n: usize, k: usize, a: &mut Vec<usize>, s: &mut Vec<usize>) {
    if t > n {
        if n % p == 0 {
            s.extend_from_slice(&a[1..=p]);
        }
    } else {
        a[t] = a[t - p];
        debruijn(t + 1, p, n, k, a, s);
        for j in (a[t - p] + 1)..k {
            a[t] = j;
            debruijn(t + 1, t, n, k, a, s);
        }
    }
}

/// The lexicographically first de Brujin word for a given alphabet and substring length.
pub struct DeBruijnWord {
    s: Vec<char>,
    idx: usize,
}

impl DeBruijnWord {
    /// Each char in alphabet is assumed to be unique and in increasing order.
    pub fn new(alphabet: &str, substring_length: usize) -> Self {
        let n = substring_length;
        let k = alphabet.chars().count();
        let mut a = vec![0; n * k];
        let mut s = Vec::new();
        debruijn(1, 1, n, k, &mut a, &mut s);

        Self {
            s: s.into_iter()
                .map(|n| alphabet.chars().nth(n).unwrap())
                .collect(),
            idx: 0,
        }
    }
}

impl Iterator for DeBruijnWord {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.s.get(self.idx).cloned();
        self.idx += 1;
        out
    }
}

crate::check_sequences!(
    DeBruijn::new_big(2), ["1", "1", "2", "16", "2048", "67108864", "144115188075855872", "1329227995784915872903807060280344576"];
    DeBruijn::<u32>::new(2), [1, 1, 2, 16, 2048, 67108864]; // check that we return the largest term we can
    DeBruijnWord::new("abcd", 2), ["a", "a", "b", "a", "c", "a", "d", "b", "b", "c", "b", "d", "c", "c", "d", "d"];
);
