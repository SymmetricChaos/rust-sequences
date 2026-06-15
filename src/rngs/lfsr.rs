use crate::rngs::UBITS;

/// A Fibonacci type linear feedback shift register that shifts bits to the right (toward the least significant bit). This iterator returns the sequence of states.
///
/// ```text
/// state = 0b0000000001, taps = 0b0100001000, bits = 10
/// 1, 512, 256, 640, 320, 672, 336, 680, 852, 938, 469, 234, 629, 826...
/// ```
pub struct FibonacciLfsr<T> {
    state: T,
    taps: T,
    bits: T,
}

impl FibonacciLfsr<u64> {
    pub fn new(state: u64, taps: u64, bits: u64) -> Self {
        assert!(bits <= 64);
        Self { state, taps, bits }
    }
}

impl Iterator for FibonacciLfsr<u64> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.state;
        let new_bit = ((self.state & self.taps).count_ones() as u64 ^ self.state) & 1;
        self.state =
            ((self.state >> 1) | (new_bit << (self.bits - 1))) & (!0 >> (UBITS - self.bits));
        Some(out)
    }
}

pub struct GaloisLfsr<T> {
    state: T,
    taps: T,
    bits: T,
}

impl GaloisLfsr<u64> {
    pub fn new(state: u64, taps: u64, bits: u64) -> Self {
        assert!(bits <= UBITS);
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
        self.state = ((self.state >> 1) | (lsb << (self.bits - 1))) & (!0 >> (UBITS - self.bits));

        Some(out)
    }
}

crate::check_sequences!(
    FibonacciLfsr::new(0b00001, 0b00100, 5),
    [
        0b00001, 0b10000, 0b01000, 0b00100, 0b10010, 0b01001, 0b10100, 0b11010, 0b01101, 0b00110,
        0b10011, 0b11001, 0b11100, 0b11110, 0b11111, 0b01111, 0b00111, 0b00011
    ];
    GaloisLfsr::new(0b10100011000000000000000001010001, 0b01000110000000000000000000000000, 32), [
            0b10100011000000000000000001010001_u64,
            0b11110010100000000000000000101000,
            0b01111001010000000000000000010100,
            0b00111100101000000000000000001010,
            0b00011110010100000000000000000101,
            0b10101100001010000000000000000010,
    ];

);

crate::sample_sequences!(
    FibonacciLfsr::new(0b0000000001, 0b0100001000, 10);
    GaloisLfsr::new(0b0000000001, 0b0100001000, 10);
);
