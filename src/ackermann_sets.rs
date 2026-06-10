use num::{BigInt, Signed, Zero};

use crate::Number;

// For 64-bit numbers only these 64 sets are used, each effectively represents a particular bit
pub const BASE_SETS: [&str; 64] = [
    "{}",
    "{{}}",
    "{{{}}}",
    "{{}{{}}}",
    "{{{{}}}}",
    "{{}{{{}}}}",
    "{{{}}{{{}}}}",
    "{{}{{}}{{{}}}}",
    "{{{}{{}}}}",
    "{{}{{}{{}}}}",
    "{{{}}{{}{{}}}}",
    "{{}{{}}{{}{{}}}}",
    "{{{{}}}{{}{{}}}}",
    "{{}{{{}}}{{}{{}}}}",
    "{{{}}{{{}}}{{}{{}}}}",
    "{{}{{}}{{{}}}{{}{{}}}}",
    "{{{{{}}}}}",
    "{{}{{{{}}}}}",
    "{{{}}{{{{}}}}}",
    "{{}{{}}{{{{}}}}}",
    "{{{{}}}{{{{}}}}}",
    "{{}{{{}}}{{{{}}}}}",
    "{{{}}{{{}}}{{{{}}}}}",
    "{{}{{}}{{{}}}{{{{}}}}}",
    "{{{}{{}}}{{{{}}}}}",
    "{{}{{}{{}}}{{{{}}}}}",
    "{{{}}{{}{{}}}{{{{}}}}}",
    "{{}{{}}{{}{{}}}{{{{}}}}}",
    "{{{{}}}{{}{{}}}{{{{}}}}}",
    "{{}{{{}}}{{}{{}}}{{{{}}}}}",
    "{{{}}{{{}}}{{}{{}}}{{{{}}}}}",
    "{{}{{}}{{{}}}{{}{{}}}{{{{}}}}}",
    "{{{}{{{}}}}}",
    "{{}{{}{{{}}}}}",
    "{{{}}{{}{{{}}}}}",
    "{{}{{}}{{}{{{}}}}}",
    "{{{{}}}{{}{{{}}}}}",
    "{{}{{{}}}{{}{{{}}}}}",
    "{{{}}{{{}}}{{}{{{}}}}}",
    "{{}{{}}{{{}}}{{}{{{}}}}}",
    "{{{}{{}}}{{}{{{}}}}}",
    "{{}{{}{{}}}{{}{{{}}}}}",
    "{{{}}{{}{{}}}{{}{{{}}}}}",
    "{{}{{}}{{}{{}}}{{}{{{}}}}}",
    "{{{{}}}{{}{{}}}{{}{{{}}}}}",
    "{{}{{{}}}{{}{{}}}{{}{{{}}}}}",
    "{{{}}{{{}}}{{}{{}}}{{}{{{}}}}}",
    "{{}{{}}{{{}}}{{}{{}}}{{}{{{}}}}}",
    "{{{{{}}}}{{}{{{}}}}}",
    "{{}{{{{}}}}{{}{{{}}}}}",
    "{{{}}{{{{}}}}{{}{{{}}}}}",
    "{{}{{}}{{{{}}}}{{}{{{}}}}}",
    "{{{{}}}{{{{}}}}{{}{{{}}}}}",
    "{{}{{{}}}{{{{}}}}{{}{{{}}}}}",
    "{{{}}{{{}}}{{{{}}}}{{}{{{}}}}}",
    "{{}{{}}{{{}}}{{{{}}}}{{}{{{}}}}}",
    "{{{}{{}}}{{{{}}}}{{}{{{}}}}}",
    "{{}{{}{{}}}{{{{}}}}{{}{{{}}}}}",
    "{{{}}{{}{{}}}{{{{}}}}{{}{{{}}}}}",
    "{{}{{}}{{}{{}}}{{{{}}}}{{}{{{}}}}}",
    "{{{{}}}{{}{{}}}{{{{}}}}{{}{{{}}}}}",
    "{{}{{{}}}{{}{{}}}{{{{}}}}{{}{{{}}}}}",
    "{{{}}{{{}}}{{}{{}}}{{{{}}}}{{}{{{}}}}}",
    "{{}{{}}{{{}}}{{}{{}}}{{{{}}}}{{}{{{}}}}}",
];

/// The Ackerman sets, a bijection between the naturals and the pure sets (aka hereditary finite sets).
///
/// ```text
/// f({}) = 0
/// f(A)  = sum 2^f(a_i) for all a_i in A
///
/// {}, {{}}, {{{}}}, {{}{{}}}, {{{{}}}}, {{}{{{}}}}, {{{}}{{{}}}}...
/// ```
pub struct AckermannSets<T> {
    ctr: T,
}

impl AckermannSets<Number> {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

#[cfg(feature = "big_int")]
impl AckermannSets<BigInt> {
    pub fn new_big() -> Self {
        Self {
            ctr: BigInt::zero(),
        }
    }
}

#[cfg(feature = "big_int")]
impl Iterator for AckermannSets<BigInt> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = number_to_set_big(&self.ctr);
        self.ctr += 1;
        Some(out)
    }

    // Optimize by skipping set generation.
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.ctr += n;
        let out = number_to_set_big(&self.ctr);
        self.ctr += 1;
        Some(out)
    }
}

impl Iterator for AckermannSets<Number> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = number_to_set_64(self.ctr as u64);
        self.ctr += 1;
        Some(out)
    }

    // Optimize by skipping set generation.
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.ctr += n as Number;
        let out = number_to_set_64(self.ctr as u64);
        self.ctr += 1;
        Some(out)
    }
}

pub fn number_to_set_64(mut n: u64) -> String {
    let mut out = String::from("{");
    for i in 0..=64 {
        if n & 1 == 1 {
            out.push_str(BASE_SETS[i]); // much faster than recursion
        }
        n >>= 1;
    }
    out.push('}');
    out
}

#[cfg(feature = "big_int")]
pub fn number_to_set_big(n: &BigInt) -> String {
    assert!(!n.is_negative());
    let mut out = String::from("{");
    for i in 0..=n.bits() {
        if i < 63 {
            if n.bit(i) {
                out.push_str(BASE_SETS[i as usize]); // much faster than recursion
            }
        } else {
            if n.bit(i) {
                out.push_str(&number_to_set_64(i));
            }
        }
    }
    out.push('}');
    out
}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[ignore = "constant generation"]
//     #[test]
//     fn generate_base_sets() {
//         println!("pub const BASE_SETS: [&str; 64] = [");
//         for i in 0..64 {
//             println!("    \"{}\",", number_to_set_64(i))
//         }
//         println!("];");
//     }
// }

crate::sample_sequences!(
    AckermannSets::new_big();
);
