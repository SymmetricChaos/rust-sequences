use crate::rngs::{UBITS, UMAX, UNumber};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LfsrType {
    Fibonacci,
    Galois,
}

/// A linear feedback shift register that shifts bits to the right (toward the least significant bit). This iterator returns the sequence of states.
///
/// ```text
/// Fibonacci type
/// state = 0b0000000001, taps = 0b1001000000
/// 1, 512, 768, 896, 960, 480, 752, 376, 700, 862, 431, 727, 875, 949...
///
/// Galois type
/// state = 0b0000000001, taps = 0b1001000000
/// 1, 800, 400, 200, 100, 50, 25, 812, 406, 203, 837, 642, 321, 896...
/// ```
pub struct Lfsr<T> {
    state: T,
    taps: T,
    bits: T,
    lfsr_type: LfsrType,
}

impl Lfsr<UNumber> {
    fn new(state: UNumber, taps: UNumber, lfsr_type: LfsrType) -> Self {
        let mut bits = 0;
        for i in 0..UBITS {
            if (taps >> i) & 1 == 1 {
                bits = i + 1;
            }
        }
        Self {
            state,
            taps,
            bits,
            lfsr_type,
        }
    }

    /// Create a Fibonacci LFSR. The number of bits used is determined from the highest set bit of the taps.
    pub fn new_fibonacci(state: UNumber, taps: UNumber) -> Self {
        Self::new(state, taps, LfsrType::Fibonacci)
    }

    /// Create a Fibonacci LFSR. The number of bits used is determined from the highest set bit of the taps.
    pub fn new_galois(state: UNumber, taps: UNumber) -> Self {
        Self::new(state, taps, LfsrType::Galois)
    }
}

impl Iterator for Lfsr<UNumber> {
    type Item = UNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.state;
        match self.lfsr_type {
            LfsrType::Fibonacci => {
                let new_bit = ((self.state & self.taps).count_ones() as UNumber ^ self.state) & 1;
                self.state = ((self.state >> 1) | (new_bit << (self.bits - 1)))
                    & (UMAX >> (UBITS - self.bits));
            }
            LfsrType::Galois => {
                let lsb = self.state & 1;
                if lsb == 1 {
                    self.state ^= self.taps;
                }
                self.state =
                    ((self.state >> 1) | (lsb << (self.bits - 1))) & (UMAX >> (UBITS - self.bits));
            }
        }

        Some(out)
    }
}

crate::sample_sequences!(
    Lfsr::new_fibonacci(0b0000000001, 0b1001000000);
    Lfsr::new_galois(0b0000000001, 0b1001000000);
);
