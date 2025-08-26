use num::{BigInt, One, Signed, Zero};

pub struct Integer {
    val: BigInt,
    ctr: BigInt,
}

impl Integer {
    pub fn new() -> Self {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::zero(),
        }
    }
}

impl Iterator for Integer {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.ctr = self.ctr.checked_add(&BigInt::one())?;
        if self.val.is_positive() {
            self.val = self.val.checked_sub(&self.ctr)?;
        } else {
            self.val = self.val.checked_add(&self.ctr)?;
        };
        Some(out)
    }
}

// mod tests {

//     #[test]
//     fn seq() {
//         use super::Integer;
//         let x = Integer::new();
//         for n in x.skip(0).take(10) {
//             println!("{n}")
//         }
//     }
// }
