use num::{BigInt, Signed};

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

/// The Ackerman sets, a bijection between the naturals and the pure sets.
/// f({}) = 0
/// f(A) = sum 2^f(a_i) for all a_i in A
pub struct AckermannSet {
    ctr: u64,
}

impl AckermannSet {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }

    // This remiains fast up to thousands of bits!
    /// The nth Ackerman set. Panics if n is negative.
    pub fn nth<T>(n: T) -> String
    where
        BigInt: From<T>,
    {
        let b = BigInt::from(n);
        assert!(!b.is_negative());
        number_to_set_big(&b)
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

pub fn number_to_set_big(n: &BigInt) -> String {
    assert!(!n.is_negative());
    let mut out = String::from("{");
    for i in 0..=n.bits() {
        if i < 64 {
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

    // #[ignore = "constant generation"]
    // #[test]
    // fn generate_base_sets() {
    //     println!("pub const BASE_SETS: [&str; 64] = [");
    //     for i in 0..64 {
    //         println!("    \"{}\",", number_to_set_64(i))
    //     }
    //     println!("];");
    // }

    #[test]
    fn compare() {
        for i in 1000..1064 {
            assert_eq!(number_to_set_64(i), number_to_set_big(&BigInt::from(i)))
        }
    }
}
