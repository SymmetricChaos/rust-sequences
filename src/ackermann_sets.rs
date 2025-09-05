// f({}) = 0
// f(A) = sum 2^f(elem_i) for all elem_i in A
// implies

use num::{BigInt, Zero};

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

pub struct AckermannSet {
    ctr: u64,
}

impl AckermannSet {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }

    pub fn nth(n: u64) -> String {
        number_to_set_64(n)
    }

    pub fn nth_big(n: BigInt) -> String {
        assert!(n >= BigInt::zero());
        number_to_set_big(n)
    }
}

impl Iterator for AckermannSet {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = number_to_set_64(self.ctr);
        self.ctr += 1;
        Some(out)
    }
}

pub fn number_to_set_64(mut n: u64) -> String {
    let mut out = String::from("{");
    for i in 0..=64 {
        if n & 1 == 1 {
            // out.push_str(&number_to_set_64(i));
            out.push_str(BASE_SETS[i]); // much faster than recursion
        }
        n >>= 1;
    }
    out.push('}');
    out
}

pub fn number_to_set_big(n: BigInt) -> String {
    assert!(n >= BigInt::zero());
    let mut out = String::from("{");
    for i in 0..=n.bits() {
        if i <= 64 {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[ignore = "constant generation"]
    #[test]
    fn generate_base_sets() {
        println!("pub const BASE_SETS: [&str; 64] = [");
        for i in 0..64 {
            println!("    \"{}\",", number_to_set_64(i))
        }
        println!("];");
    }

    #[test]
    fn compare() {
        for i in 100..164 {
            assert_eq!(number_to_set_64(i), number_to_set_big(BigInt::from(i)))
        }
    }
}
