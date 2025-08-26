use num::{CheckedAdd, One, Zero};

pub struct Lucas<T> {
    a: T,
    b: T,
}

impl<T: CheckedAdd + Clone + One + Zero> Lucas<T> {
    pub fn new() -> Self {
        Self {
            a: T::one() + T::one(),
            b: T::one(),
        }
    }
}

impl<T: CheckedAdd + Clone + One + Zero> Iterator for Lucas<T> {
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
        use super::Lucas;
        use num::BigUint;
        let x = Lucas::<BigUint>::new();
        for n in x.skip(0).take(10) {
            println!("{n}")
        }
    }
}
