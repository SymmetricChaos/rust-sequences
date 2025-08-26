use num::{BigInt, One};

pub struct Powers {
    val: BigInt,
    pow: BigInt,
}

impl Powers {
    pub fn new(pow: u32) -> Self {
        Self {
            val: BigInt::one(),
            pow: BigInt::from(pow),
        }
    }
}

impl Iterator for Powers {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val.clone();
        self.val = self.val.checked_mul(&self.pow)?;
        Some(out)
    }
}

// mod tests {

//     #[test]
//     fn seq() {
//         use super::Powers;
//         let x = Powers::new(3.try_into().unwrap());
//         for n in x.skip(0).take(10) {
//             println!("{n}")
//         }
//     }
// }
