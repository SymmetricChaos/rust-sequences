#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scrambler {
    Plus,
    PlusPlus,
    StarStar,
}

/// The Xoroshiro128 PRNG.
pub struct Xoroshiro128 {
    state: [u64; 2],
    scrambler: Scrambler,
}

impl Xoroshiro128 {
    /// Initalize Xoroshiro128+
    pub fn new_plus(state: [u64; 2]) -> Self {
        Self {
            state,
            scrambler: Scrambler::Plus,
        }
    }

    /// Initalize Xoroshiro128++
    pub fn new_plus_plus(state: [u64; 2]) -> Self {
        Self {
            state,
            scrambler: Scrambler::PlusPlus,
        }
    }

    /// Initalize Xoroshiro128**
    pub fn new_star_star(state: [u64; 2]) -> Self {
        Self {
            state,
            scrambler: Scrambler::StarStar,
        }
    }

    fn step(&mut self) {
        if self.scrambler == Scrambler::PlusPlus {
            self.state[1] ^= self.state[0];
            self.state[0] = self.state[0].rotate_left(49) ^ self.state[1] ^ (self.state[1] << 21);
            self.state[1] = self.state[1].rotate_left(28);
        } else {
            self.state[1] ^= self.state[0];
            self.state[0] = self.state[0].rotate_left(24) ^ self.state[1] ^ (self.state[1] << 16);
            self.state[1] = self.state[1].rotate_left(37);
        }
    }

    const JUMP: [u64; 2] = [0xdf900294d8f554a5, 0x170865df4b3201fc];
    const LONG_JUMP: [u64; 2] = [0xd2a98b26625eee7b, 0xdddf9b1090aa7ac1];

    const JUMP_PP: [u64; 2] = [0x2bd7a6a6e99c2ddc, 0x0992ccaf6a6fca05];
    const LONG_JUMP_PP: [u64; 2] = [0x360fd5f2cf8d5d99, 0x9c6e6877736c46e3];

    /// Jump forward by 2^64 steps.
    pub fn jump(&mut self) {
        let mut s = [0; 4];
        if self.scrambler == Scrambler::PlusPlus {
            for j in Self::JUMP_PP {
                for b in 0..64 {
                    if j & (1 << b) != 0 {
                        for n in 0..4 {
                            s[n] ^= self.state[n]
                        }
                    }
                    self.step()
                }
            }
        } else {
            for j in Self::JUMP {
                for b in 0..64 {
                    if j & (1 << b) != 0 {
                        for n in 0..4 {
                            s[n] ^= self.state[n]
                        }
                    }
                    self.step()
                }
            }
        }
        for n in 0..4 {
            self.state[n] = s[n];
        }
    }

    /// Jump forward by 2^96 steps.
    pub fn long_jump(&mut self) {
        let mut s = [0; 4];
        if self.scrambler == Scrambler::PlusPlus {
            for j in Self::LONG_JUMP_PP {
                for b in 0..64 {
                    if j & (1 << b) != 0 {
                        for n in 0..4 {
                            s[n] ^= self.state[n]
                        }
                    }
                    self.step()
                }
            }
        } else {
            for j in Self::LONG_JUMP {
                for b in 0..64 {
                    if j & (1 << b) != 0 {
                        for n in 0..4 {
                            s[n] ^= self.state[n]
                        }
                    }
                    self.step()
                }
            }
        }

        for n in 0..4 {
            self.state[n] = s[n];
        }
    }
}

impl Iterator for Xoroshiro128 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = match self.scrambler {
            Scrambler::PlusPlus => (self.state[0].wrapping_add(self.state[1]))
                .rotate_left(17)
                .wrapping_add(self.state[0]),
            Scrambler::StarStar => (self.state[0].wrapping_mul(5))
                .rotate_left(7)
                .wrapping_mul(9),
            Scrambler::Plus => self.state[0].wrapping_add(self.state[1]),
        };

        self.step();
        Some(out)
    }
}
