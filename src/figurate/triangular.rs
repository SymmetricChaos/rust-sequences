use num::{BigInt, CheckedAdd, FromPrimitive, One, Signed, Zero};

/// The triangular numbers.
/// 0, 1, 3, 6, 10, 15, 21, 28, 36, 45...
pub struct Triangular<T> {
    val: T,
    ctr: T,
}

impl<T: Clone + CheckedAdd + One + Zero> Triangular<T> {
    pub fn new() -> Self {
        Self {
            val: T::zero(),
            ctr: T::one(),
        }
    }
}

impl Triangular<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }

    pub fn nth<T>(n: T) -> BigInt
    where
        BigInt: From<T>,
    {
        let n = BigInt::from(n);
        assert!(!n.is_negative());
        (n.clone() * (n + 1)) / BigInt::from_i32(2).unwrap()
    }
}

impl<T: Clone + CheckedAdd + One> Iterator for Triangular<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

crate::print_sequences!(
    Triangular::<i32>::new(), skip 0, 10;
    Triangular::<i32>::new(), skip 1, 10;
    Triangular::<i32>::new(), skip 2, 10;
    [0,1,2,3,10,25,100].into_iter().map(|x| Triangular::nth(x)), 10;
);

crate::check_sequences!(
    Triangular::new_big(), [0, 1, 3, 6, 10, 15, 21, 28, 36, 45];
);
