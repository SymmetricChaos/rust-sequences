#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PcgTransform {
    /// Random Shift
    Rs,
    /// Random Rotation
    Rr,
    // Xorshift with Random Rotation
    XshRr,
    // Xorshift with Random Shift
    XshRs,
}

impl PcgTransform {
    pub fn apply(&self, n: u64) -> u64 {
        match self {
            Self::Rs => n >> (29 - (n >> 61)),
            Self::Rr => u64::rotate_right(n, 29 - (n >> 61) as u32),
            Self::XshRr => u64::rotate_right((n ^ (n >> 18)) >> 27, (n >> 59) as u32),
            Self::XshRs => (n ^ (n >> 22)) >> (22 + (n >> 61)),
        }
    }
}

/// A Permuted Congruential Generator with a 64-bit state and a 32-bit output.
pub struct Pcg64_32 {
    state: u64,
    multiplier: u64,
    increment: u64,
    transform: PcgTransform,
}

impl Pcg64_32 {
    /// Output permutation is xorshift wiyj random rotation.
    pub fn new_xsh_rr(seed: u64, multiplier: u64, increment: u64) -> Self {
        Self {
            state: seed,
            multiplier,
            increment,
            transform: PcgTransform::XshRr,
        }
    }

    /// Output permutation is xorshift with random shift.
    pub fn new_xsh_rs(seed: u64, multiplier: u64, increment: u64) -> Self {
        Self {
            state: seed,
            multiplier,
            increment,
            transform: PcgTransform::XshRs,
        }
    }
}

impl Iterator for Pcg64_32 {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.state;
        self.state = self
            .state
            .wrapping_mul(self.multiplier)
            .wrapping_add(self.increment);
        Some(self.transform.apply(x) as u32)
    }
}
