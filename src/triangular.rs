use num::{BigInt, One, Zero};

pub struct Triangular {
    val: BigInt,
    ctr: BigInt,
}

impl Triangular {
    pub fn new() -> Self {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::one(),
        }
    }
}

impl Iterator for Triangular {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&BigInt::one())?;
        Some(out)
    }
}

// mod tests {

//     #[test]
//     fn seq() {
//         use super::Triangular;
//         let x = Triangular::new();
//         for n in x.skip(10).take(10) {
//             println!("{n}")
//         }
//     }
// }
