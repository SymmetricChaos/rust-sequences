use num::{CheckedAdd, CheckedMul, One, Zero};

pub struct Factorial<T> {
    val: T,
    ctr: T,
}

impl<T: CheckedAdd + Clone + One + Zero> Factorial<T> {
    pub fn new() -> Self {
        Self {
            val: T::one(),
            ctr: T::one() + T::one(),
        }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + One + Zero> Iterator for Factorial<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&T::one())?;
        Some(out)
    }
}

mod tests {

    #[test]
    fn seq() {
        use super::Factorial;
        use num::BigUint;
        let x = Factorial::<BigUint>::new();
        for n in x.skip(0).take(10) {
            println!("{n}")
        }
    }
}
