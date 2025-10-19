use num::{BigInt, CheckedAdd};

/// Arithmetic sequence with chosen initial value and increment
pub struct Arithmetic<T> {
    val: T,
    inc: T,
}

impl<T> Arithmetic<T> {
    pub fn new(init: T, inc: T) -> Self {
        Self { val: init, inc }
    }
}

impl Arithmetic<BigInt> {
    pub fn new_big<G>(init: G, inc: G) -> Self
    where
        BigInt: From<G>,
    {
        Self {
            val: BigInt::from(init),
            inc: BigInt::from(inc),
        }
    }
}

impl<T: Clone + CheckedAdd> Iterator for Arithmetic<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.inc)?;
        Some(out)
    }
}

crate::check_iteration_times!(
    Arithmetic::new_big(4, 3), 3_500_000;
    Arithmetic::<u64>::new(4, 3), 3_500_000;
);

crate::check_sequences!(
    Arithmetic::new_big(0, 0), 0, 10, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    Arithmetic::new_big(1, 1), 0, 10, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    Arithmetic::new_big(3, 2), 0, 10, [3, 5, 7, 9, 11, 13, 15, 17, 19, 21];
    Arithmetic::new_big(4, 3), 0, 10, [4, 7, 10, 13, 16, 19, 22, 25, 28, 31];
);
