use num::{BigInt, CheckedAdd, Integer, Zero};

/// The factoradic representation of each non-negative integer. Panics if n is less than zero.
/// When size is 0 the minimal length vector is returned.
/// For any other value of size the output is padded to the left with zeroes to equal the size and returns None of the output is greater than the factorial of size.
pub fn factoradic<N: Integer>(mut n: N, size: usize) -> Option<Vec<N>> {
    assert!(n >= N::zero());
    let mut out = vec![N::zero()];
    let mut divisor = N::one() + N::one();
    while !n.is_zero() {
        let (q, r) = n.div_rem(&divisor);
        out.push(r);
        n = q;
        divisor = divisor + N::one();
    }
    if out.len() > size && size != 0 {
        return None;
    }
    while out.len() < size {
        out.push(N::zero());
    }
    Some(out.into_iter().rev().collect())
}

/// The factoradic representation of each non-negative integer.
/// When size is 0 the minimal length vector is returned and values are returned until the counter is exhausted.
/// For any other value of size the output is padded to the left with zeroes to equal the size and terminates at the factorial of size.
pub struct Factoradic<T> {
    ctr: T,
    size: usize,
}

impl<T: CheckedAdd + Clone + Integer + Zero> Factoradic<T> {
    pub fn new(size: usize) -> Self {
        Self {
            ctr: T::zero(),
            size,
        }
    }
}

impl Factoradic<BigInt> {
    pub fn new_big(size: usize) -> Self {
        Self {
            ctr: BigInt::zero(),
            size,
        }
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for Factoradic<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = factoradic(self.ctr.clone(), self.size);
        self.ctr = self.ctr.checked_add(&T::one())?;
        out
    }
}

crate::print_sequences!(
    Factoradic::new_big(0), 10, "{:?}", "\n";
    Factoradic::new_big(1), 0, 10, "{:?}", "\n";
    Factoradic::new_big(2), 0, 10, "{:?}", "\n";
    Factoradic::new_big(3), 0, 10, "{:?}", "\n";
);
