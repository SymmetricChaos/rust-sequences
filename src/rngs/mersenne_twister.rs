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

/// The Mersenne Twister.
///
/// ```text
/// seed = 5489
/// 3499211612, 581869302, 3890346734, 3586334585, 545404204...
/// ```
pub struct Mt19937 {
    index: usize,
    array: [u32; N32],
}

impl Mt19937 {
    pub fn new(seed: u32) -> Self {
        let mut array = [0u32; N32];
        let index = N32;
        array[0] = seed;
        for i in 1..N32 {
            array[i] = 1812433253_u32
                .wrapping_mul(array[i - 1] ^ (array[i - 1] >> 30))
                .wrapping_add(i as u32)
        }
        Self { index, array }
    }

    pub fn from_array(key: &[u32]) -> Self {
        let mut rng = Self::new(19650218);
        let mut i = 1;
        let mut j = 0;
        for _ in 0..std::cmp::max(N32, key.len()) {
            rng.array[i] = (rng.array[i]
                ^ ((rng.array[i - 1] ^ (rng.array[i - 1] >> 30)).wrapping_mul(1664525)))
            .wrapping_add(key[j])
            .wrapping_add(j as u32);
            i += 1;
            if i >= N32 {
                rng.array[0] = rng.array[N32 - 1];
                i = 1;
            }
            j += 1;
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..N32 - 1 {
            rng.array[i] = (rng.array[i]
                ^ ((rng.array[i - 1] ^ (rng.array[i - 1] >> 30)).wrapping_mul(1566083941)))
            .wrapping_sub(i as u32);
            i += 1;
            if i >= N32 {
                rng.array[0] = rng.array[N32 - 1];
                i = 1;
            }
        }
        rng.array[0] = 1 << 31;
        rng
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

/// The 64-bit version of the Mersenne Twister.
///
/// ```text
/// seed = 5489
/// 14514284786278117030, 4620546740167642908, 13109570281517897720...
/// ```
pub struct Mt19937_64 {
    index: usize,
    array: [u64; N64],
}

impl Mt19937_64 {
    pub fn new(seed: u64) -> Self {
        let mut array = [0u64; N64];
        let index = N64;
        array[0] = seed;
        for i in 1..N64 {
            array[i] = 6364136223846793005_u64
                .wrapping_mul(array[i - 1] ^ (array[i - 1] >> 62))
                .wrapping_add(i as u64)
        }
        Self { index, array }
    }

    pub fn from_array(key: &[u64]) -> Self {
        let mut rng = Self::new(19650218);
        let mut i = 1;
        let mut j = 0;
        for _ in 0..std::cmp::max(N64, key.len()) {
            rng.array[i] = (rng.array[i]
                ^ ((rng.array[i - 1] ^ (rng.array[i - 1] >> 62))
                    .wrapping_mul(3935559000370003845)))
            .wrapping_add(key[j])
            .wrapping_add(j as u64);
            i += 1;
            if i >= N64 {
                rng.array[0] = rng.array[N64 - 1];
                i = 1;
            }
            j += 1;
            if j >= key.len() {
                j = 0;
            }
        }
        for _ in 0..N64 - 1 {
            rng.array[i] = (rng.array[i]
                ^ ((rng.array[i - 1] ^ (rng.array[i - 1] >> 62))
                    .wrapping_mul(2862933555777941757)))
            .wrapping_sub(i as u64);
            i += 1;
            if i >= N64 {
                rng.array[0] = rng.array[N64 - 1];
                i = 1;
            }
        }
        rng.array[0] = 1 << 63;
        rng
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

crate::check_sequences!(
    Mt19937::new(5489), [3499211612_u32, 581869302, 3890346734, 3586334585, 545404204, 4161255391, 3922919429, 949333985, 2715962298, 1323567403, 418932835, 2350294565, 1196140740, 809094426, 2348838239, 4264392720, 4112460519, 4279768804, 4144164697, 4156218106, 676943009, 3117454609];
    Mt19937_64::new(5489), [14514284786278117030_u64, 4620546740167642908, 13109570281517897720, 17462938647148434322, 355488278567739596, 7469126240319926998, 4635995468481642529, 418970542659199878, 9604170989252516556, 6358044926049913402, 5058016125798318033, 10349215569089701407];
);

crate::sample_sequences!(
    Mt19937::new(5489);
    Mt19937_64::new(5489);
);
