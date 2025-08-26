use num::{CheckedAdd, One, Zero};

pub struct Triangular<T> {
    val: T,
    ctr: T,
}

impl<T: CheckedAdd + Clone + One + Zero> Triangular<T> {
    pub fn new() -> Self {
        Self {
            val: T::zero(),
            ctr: T::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + One + Zero> Iterator for Triangular<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.ctr = self.ctr.checked_add(&T::one())?;
        self.val = self.val.checked_add(&self.ctr)?;
        Some(out)
    }
}

mod tests {

    #[test]
    fn seq() {
        use super::Triangular;
        use num::BigUint;
        let x = Triangular::<BigUint>::new();
        for n in x.skip(10).take(10) {
            println!("{n}")
        }
    }
}
