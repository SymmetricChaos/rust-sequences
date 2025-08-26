use num::{BigInt, One};

pub struct Factorial {
    val: BigInt,
    ctr: BigInt,
}

impl Factorial {
    pub fn new() -> Self {
        Self {
            val: BigInt::one(),
            ctr: BigInt::from(2),
        }
    }
}

impl Iterator for Factorial {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&BigInt::one())?;
        Some(out)
    }
}

// mod tests {

//     #[test]
//     fn seq() {
//         use super::Factorial;
//         let x = Factorial::new();
//         for n in x.skip(0).take(10) {
//             println!("{n}")
//         }
//     }
// }
