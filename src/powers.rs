use num::{CheckedAdd, CheckedMul, One, Zero};

pub struct Powers<T> {
    val: T,
    pow: T,
}

impl<T: CheckedMul + Clone + One + Zero> Powers<T> {
    pub fn new(pow: T) -> Self {
        Self { val: T::one(), pow }
    }
}

impl<T: CheckedAdd + CheckedMul + Clone + One + Zero> Iterator for Powers<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.pow)?;
        Some(out)
    }
}

mod tests {

    #[test]
    fn seq() {
        use super::Powers;
        use num::BigUint;
        let x = Powers::<BigUint>::new(3.try_into().unwrap());
        for n in x.skip(0).take(10) {
            println!("{n}")
        }
    }
}
