const N32: usize = 624;
const M32: usize = 397;
const A32: u32 = 0x9908b0df;
const UPPER_MASK32: u32 = 0x80000000;
const LOWER_MASK32: u32 = 0x7fffffff;

const N64: usize = 312;
const M64: usize = 156;
const A64: u64 = 0xb5026f5aa96619e9;
const UPPER_MASK64: u64 = 0xffffffff80000000;
const LOWER_MASK64: u64 = 0x000000007fffffff;

pub struct Mt19937 {
    index: usize,
    array: [u32; N32],
}

impl Mt19937 {
    pub fn new(init: u32) -> Self {
        todo!()
    }

    fn twist(&mut self) {
        for i in 0..N32 - M32 {
            let x = (self.array[i] & UPPER_MASK32) | (self.array[i + 1] & LOWER_MASK32);
            self.array[i] = self.array[i + M32] ^ (x >> 1) ^ ((x & 1).wrapping_mul(A32));
        }
        for i in N32 - M32..N32 - 1 {
            let x = (self.array[i] & UPPER_MASK32) | (self.array[i + 1] & LOWER_MASK32);
            self.array[i] = self.array[i + M32 - N32] ^ (x >> 1) ^ ((x & 1).wrapping_mul(A32));
        }
        let x = (self.array[N32 - 1] & UPPER_MASK32) | (self.array[0] & LOWER_MASK32);
        self.array[N32 - 1] = self.array[M32 - 1] ^ (x >> 1) ^ ((x & 1).wrapping_mul(A32));
    }

    fn temper(mut x: u32) -> u32 {
        x ^= x >> 11;
        x ^= (x << 7) & 0x9d2c5680;
        x ^= (x << 15) & 0xefc60000;
        x ^= x >> 18;
        x
    }
}

impl Iterator for Mt19937 {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= N32 {
            self.twist();
            self.index = 0;
        }
        let y = self.array[self.index];
        self.index += 1;
        Some(Self::temper(y))
    }
}

pub struct Mt19937_64 {
    index: usize,
    array: [u64; N64],
}

impl Mt19937_64 {
    pub fn new(init: u64) -> Self {
        todo!()
    }

    fn twist(&mut self) {
        for i in 0..N64 - M64 {
            let x = (self.array[i] & UPPER_MASK64) | (self.array[i + 1] & LOWER_MASK64);
            self.array[i] = self.array[i + M64] ^ (x >> 1) ^ ((x & 1).wrapping_mul(A64));
        }
        for i in N64 - M64..N64 - 1 {
            let x = (self.array[i] & UPPER_MASK64) | (self.array[i + 1] & LOWER_MASK64);
            self.array[i] = self.array[i + M64 - N64] ^ (x >> 1) ^ ((x & 1).wrapping_mul(A64));
        }
        let x = (self.array[N64 - 1] & UPPER_MASK64) | (self.array[0] & LOWER_MASK64);
        self.array[N64 - 1] = self.array[M64 - 1] ^ (x >> 1) ^ ((x & 1).wrapping_mul(A64));
    }

    fn temper(mut x: u64) -> u64 {
        x ^= (x >> 29) & 0x5555555555555555;
        x ^= (x << 17) & 0x71d67fffeda60000;
        x ^= (x << 37) & 0xfff7eee000000000;
        x ^= x >> 43;
        x
    }
}

impl Iterator for Mt19937_64 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= N64 {
            self.twist();
            self.index = 0;
        }
        let y = self.array[self.index];
        self.index += 1;
        Some(Self::temper(y))
    }
}
