// use std::hash::Hash;

// use num::{BigInt, CheckedAdd, One, PrimInt, Zero, rational::Ratio};

// use crate::core::Primes;

// // Fraction of integers divisible by 2 is 1/2
// // Fraction of integers divisible by 2 or 3 is 1/2 + 1/3 - 1/6 = 3/6 + 2/6 - 1/6 = 4/6 = 2/3
// // Fraction of integers divisible by 2 or 3 or 5 is 1/2 + 1/3 + 1/5 - 1/6 - 1/10 - 1/15 + 1/30 = 15/30 + 10/30 + 6/30 - 5/30 - 3/30 - 2/30 + 1/30 = 22/30 = 11/15

// /// Fraction of integes divisible by each prime from 2 onward.
// /// 1/2, 2/3, 11/15,
// pub struct PrimeFactorDensity<T> {
//     primes: Primes<T>,
//     saved_primes: Vec<T>,
// }

// impl<T: PrimInt> PrimeFactorDensity<T> {
//     pub fn new() -> Self {
//         Self {
//             primes: Primes::<T>::new(),
//             saved_primes: Vec::new(),
//         }
//     }
// }

// impl PrimeFactorDensity<BigInt> {
//     pub fn new_big() -> Self {
//         Self {
//             primes: Primes::new_big(),
//             saved_primes: Vec::new(),
//         }
//     }
// }

// impl<T: CheckedAdd + Clone + Eq + Hash + One + Zero> Iterator for PrimeFactorDensity<T> {
//     type Item = Ratio<T>;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.saved_primes.push(self.primes.next()?);

//         for i in self.saved_primes {}
//     }
// }
