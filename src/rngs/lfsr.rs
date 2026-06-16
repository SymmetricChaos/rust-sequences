use crate::rngs::{UBITS, UMAX};

/// A Fibonacci type linear feedback shift register that shifts bits to the right (toward the least significant bit). This iterator returns the sequence of states.
///
/// ```text
/// state = 0b0000000001, taps = 0b1001000000
/// 1, 512, 768, 896, 960, 480, 752, 376, 700, 862, 431, 727, 875, 949...
/// ```
pub struct FibonacciLfsr<T> {
    state: T,
    taps: T,
    bits: T,
}

impl FibonacciLfsr<u64> {
    /// Create an LFSR. The number of bits used is determined from the taps.
    pub fn new(state: u64, taps: u64) -> Self {
        let mut bits = 0;
        for i in 0..UBITS {
            if (taps >> i) & 1 == 1 {
                bits = i + 1;
            }
        }
        Self { state, taps, bits }
    }

    /// Create an LFSR with a specified number of bits used.
    pub fn new_with_bits(state: u64, taps: u64, bits: u64) -> Self {
        Self { state, taps, bits }
    }
}

impl Iterator for FibonacciLfsr<u64> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.state;
        let new_bit = ((self.state & self.taps).count_ones() as u64 ^ self.state) & 1;
        self.state =
            ((self.state >> 1) | (new_bit << (self.bits - 1))) & (UMAX >> (UBITS - self.bits));
        Some(out)
    }
}

/// A Galois type linear feedback shift register that shifts bits to the right (toward the least significant bit). This iterator returns the sequence of states.
///
/// ```test
/// state = 0b0000000001, taps = 0b1001000000
/// 1, 800, 400, 200, 100, 50, 25, 812, 406, 203, 837, 642, 321, 896...
/// ```
pub struct GaloisLfsr<T> {
    state: T,
    taps: T,
    bits: T,
}

impl GaloisLfsr<u64> {
    /// Create an LFSR. The number of bits used is determined from the taps.
    pub fn new(state: u64, taps: u64) -> Self {
        let mut bits = 0;
        for i in 0..UBITS {
            if (taps >> i) & 1 == 1 {
                bits = i + 1;
            }
        }
        Self { state, taps, bits }
    }

    /// Create an LFSR with a specified number of bits used.
    pub fn new_with_bits(state: u64, taps: u64, bits: u64) -> Self {
        Self { state, taps, bits }
    }
}

impl Iterator for GaloisLfsr<u64> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.state;
        let lsb = self.state & 1;
        if lsb == 1 {
            self.state ^= self.taps;
        }
        self.state = ((self.state >> 1) | (lsb << (self.bits - 1))) & (UMAX >> (UBITS - self.bits));

        Some(out)
    }
}

crate::check_sequences!(
    FibonacciLfsr::new_with_bits(0b00001, 0b00100, 5),
    [
        0b00001, 0b10000, 0b01000, 0b00100, 0b10010, 0b01001, 0b10100, 0b11010, 0b01101, 0b00110,
        0b10011, 0b11001, 0b11100, 0b11110, 0b11111, 0b01111, 0b00111, 0b00011
    ];
    GaloisLfsr::new_with_bits(0b10100011000000000000000001010001, 0b01000110000000000000000000000000, 32), [
            0b10100011000000000000000001010001_u64,
            0b11110010100000000000000000101000,
            0b01111001010000000000000000010100,
            0b00111100101000000000000000001010,
            0b00011110010100000000000000000101,
            0b10101100001010000000000000000010,
    ];

);

crate::sample_sequences!(
    FibonacciLfsr::new(0b0000000001, 0b1001000000);
    GaloisLfsr::new(0b0000000001, 0b1001000000);
);
