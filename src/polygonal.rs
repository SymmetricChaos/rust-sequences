use num::{CheckedAdd, One, Zero};

pub struct Polygonal<T> {
    val: T,
    ctr: T,
    inc: T,
}

impl<T: CheckedAdd + Clone + One + Zero> Polygonal<T> {
    /// Create a sequence of polygonal numbers starting from zero.
    /// n = 0 -> natural numbers
    /// n = 1 -> triangular numbers
    /// n = 2 -> square numbers
    /// and then higher order polygonal numbers
    pub fn new(n: T) -> Self {
        Self {
            val: T::zero(),
            ctr: T::one(),
            inc: n,
        }
    }
}

impl<T: CheckedAdd + Clone + One + Zero> Iterator for Polygonal<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&self.inc)?;
        Some(out)
    }
}

mod tests {

    #[test]
    fn seq() {
        use super::Polygonal;
        use num::BigUint;
        let x = Polygonal::<BigUint>::new(2.try_into().unwrap());
        for n in x.skip(0).take(10) {
            println!("{n}")
        }
    }
}
