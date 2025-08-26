use num::{BigInt, One};

pub struct Lucas {
    a: BigInt,
    b: BigInt,
}

impl Lucas {
    pub fn new() -> Self {
        Self {
            a: BigInt::from(2),
            b: BigInt::one(),
        }
    }
}

impl Iterator for Lucas {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.a.clone();
        let t = self.a.checked_add(&self.b)?;
        self.a = self.b.clone();
        self.b = t;
        Some(out)
    }
}

// mod tests {

//     #[test]
//     fn seq() {
//         use super::Lucas;
//         let x = Lucas::new();
//         for n in x.skip(0).take(10) {
//             println!("{n}")
//         }
//     }
// }
