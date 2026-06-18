#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PcgTransform {
    // Xorshift with Random Rotation
    XshRr,
    // Xorshift with Random Shift
    XshRs,
}

impl PcgTransform {
    pub fn apply(&self, n: u64) -> u64 {
        match self {
            Self::XshRr => u64::rotate_right((n ^ (n >> 18)) >> 27, (n >> 59) as u32),
            Self::XshRs => (n ^ (n >> 22)) >> (22 + (n >> 61)),
        }
    }
}

/// A Permuted Congruential Generator with a 64-bit state and a 32-bit output.
///
/// ```text
/// Xorshift with Random Rotation (recommended settings)
/// seed = 0, multiplier = 6364136223846793005, increment = 1442695040888963407
/// 0, 2687235069, 1747165774, 158681, 13462534, 12872360, 588101...
///
/// Xorshift with Random Shift (recommended settings)
/// seed = 0, multiplier = 6364136223846793005, increment = 1442695040888963407
/// 0, 367836042, 599385756, 3181286013, 1527626195, 447129947...
/// ```
pub struct Pcg64_32 {
    state: u64,
    multiplier: u64,
    increment: u64,
    transform: PcgTransform,
}

impl Pcg64_32 {
    /// Output permutation is xorshift with random rotation.
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

crate::sample_sequences!(
    Pcg64_32::new_xsh_rr(0, 6364136223846793005, 1442695040888963407);
    Pcg64_32::new_xsh_rs(0, 6364136223846793005, 1442695040888963407);
);
