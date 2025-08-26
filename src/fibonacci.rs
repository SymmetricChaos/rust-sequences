use num::{CheckedAdd, One, Zero};

pub struct Fibonacci<T> {
    a: T,
    b: T,
}

impl<T: CheckedAdd + Clone + One + Zero> Fibonacci<T> {
    pub fn new() -> Self {
        Self {
            a: T::zero(),
            b: T::one(),
        }
    }
}

impl<T: CheckedAdd + Clone + One + Zero> Iterator for Fibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&self.b)?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

mod tests {

    #[test]
    fn seq() {
        use super::Fibonacci;
        use num::BigUint;
        let x = Fibonacci::<BigUint>::new();
        for n in x.skip(0).take(10) {
            println!("{n}")
        }
    }
}
