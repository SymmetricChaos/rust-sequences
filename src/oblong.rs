use num::{BigInt, Zero};

pub struct Oblong {
    val: BigInt,
    ctr: BigInt,
}

impl Oblong {
    pub fn new() -> Self {
        Self {
            val: BigInt::zero(),
            ctr: BigInt::from(2),
        }
    }
}

impl Iterator for Oblong {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_add(&self.ctr)?;
        self.ctr = self.ctr.checked_add(&BigInt::from(2))?;
        Some(out)
    }
}

// mod tests {

//     #[test]
//     fn seq() {
//         use super::Oblong;
//         let x = Oblong::new();
//         for n in x.skip(0).take(10) {
//             println!("{n}")
//         }
//     }
// }
