use num::{CheckedAdd, CheckedSub, One, Signed, Zero};

pub struct Integer<T> {
    val: T,
    ctr: T,
}

impl<T: CheckedAdd + Clone + One + Signed + Zero> Integer<T> {
    pub fn new() -> Self {
        Self {
            val: T::zero(),
            ctr: T::zero(),
        }
    }
}

impl<T: CheckedAdd + CheckedSub + Clone + One + Signed + Zero> Iterator for Integer<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.ctr = self.ctr.checked_add(&T::one())?;
        if self.val.is_positive() {
            self.val = self.val.checked_sub(&self.ctr)?;
        } else {
            self.val = self.val.checked_add(&self.ctr)?;
        };
        Some(out)
    }
}

mod tests {

    #[test]
    fn seq() {
        use super::Integer;
        use num::BigInt;
        let x = Integer::<BigInt>::new();
        for n in x.skip(0).take(10) {
            println!("{n}")
        }
    }
}
