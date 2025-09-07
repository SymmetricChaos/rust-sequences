// use num::{BigInt, One, Zero};

// pub struct Partition {
//     values: Vec<BigInt>,
//     ctr: usize,
// }

// impl Partition {
//     pub fn new() -> Self {
//         Self {
//             values: vec![BigInt::one()],
//             ctr: 0,
//         }
//     }
// }

// impl Iterator for Partition {
//     type Item = BigInt;

//     fn next(&mut self) -> Option<Self::Item> {
//         let out = self.values[self.ctr].clone();
//         self.ctr += 1;

//         let mut parts = BigInt::zero();
//         let mut ints = crate::core::Integer::new();
//         let mut pentag = 3;

//         while idx <= self.ctr {
//             parts += self.values[self.ctr - idx].clone();
//             ints.next()
//         }

//         Some(out)
//     }
// }

// crate::check_sequences!(
//     Partition::new(), 0, 10, [1, 1, 2, 3, 5, 7, 11, 15, 22, 30];
// );
