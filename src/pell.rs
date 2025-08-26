use num::{BigInt, One, Zero};

pub struct Pell {
    a: BigInt,
    b: BigInt,
}

impl Pell {
    pub fn new() -> Self {
        Self {
            a: BigInt::zero(),
            b: BigInt::one(),
        }
    }
}

impl Iterator for Pell {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let double_b = self.b.checked_mul(&BigInt::from(2))?;
        let t = self.a.checked_add(&double_b)?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

// mod tests {

//     #[test]
//     fn seq() {
//         use super::Pell;
//         let x = Pell::new();
//         for n in x.skip(0).take(10) {
//             println!("{n}")
//         }
//     }
// }
