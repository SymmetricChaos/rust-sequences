use num::{BigInt, CheckedAdd, Integer, Zero};

pub fn factoradic<N: Integer>(mut n: N) -> Vec<N> {
    assert!(n >= N::zero());
    let mut out = vec![N::zero()];
    let mut divisor = N::one() + N::one();
    while !n.is_zero() {
        let (q, r) = n.div_rem(&divisor);
        out.push(r);
        n = q;
        divisor = divisor + N::one();
    }
    out.into_iter().rev().collect()
}

pub fn factoradic_permutation<N: Integer>(mut n: N, size: usize) -> Option<Vec<N>> {
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

/// The factoradic representation of each non-negative integer. Also known as the factorial base representation. These can be used to define an ordering for permutations.
pub struct Factoradic<T> {
    ctr: T,
    size: usize,
}

impl<T: Zero> Factoradic<T> {
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

impl<T: Clone + CheckedAdd + Integer> Iterator for Factoradic<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = factoradic_permutation(self.ctr.clone(), self.size);
        self.ctr = self.ctr.checked_add(&T::one())?;
        out
    }
}

crate::print_values!(
    print_array, formatter "{:?}", sep "\n";
    Factoradic::new_big(0), 0, 12;
    Factoradic::new_big(3), 0, 12;
);
