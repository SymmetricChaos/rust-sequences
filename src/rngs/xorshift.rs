#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum XorshiftRule {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
}

impl XorshiftRule {
    fn apply(&self, state: &mut u64, triple: (u64, u64, u64)) {
        let (a, b, c) = triple;
        match self {
            XorshiftRule::A0 => {
                *state ^= *state << a;
                *state ^= *state >> b;
                *state ^= *state << c;
            }
            XorshiftRule::A1 => {
                *state ^= *state >> a;
                *state ^= *state << b;
                *state ^= *state >> c;
            }
            XorshiftRule::A2 => {
                *state ^= *state << c;
                *state ^= *state >> b;
                *state ^= *state << a;
            }
            XorshiftRule::A3 => {
                *state ^= *state >> c;
                *state ^= *state << b;
                *state ^= *state >> a;
            }
            XorshiftRule::A4 => {
                *state ^= *state << a;
                *state ^= *state << c;
                *state ^= *state >> b;
            }
            XorshiftRule::A5 => {
                *state ^= *state >> a;
                *state ^= *state >> c;
                *state ^= *state << b;
            }
            XorshiftRule::A6 => {
                *state ^= *state >> b;
                *state ^= *state << a;
                *state ^= *state << c;
            }
            XorshiftRule::A7 => {
                *state ^= *state << b;
                *state ^= *state >> a;
                *state ^= *state >> c;
            }
        }
    }
}

// values for the Star scramblers From Vigna
const M32: u64 = 0x2545F4914F6CDD1D;
const M8: u64 = 0x106689D45497FDB5;
const M2: u64 = 0x74321163EEC4A005;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scrambler {
    None,
    Plus,
    Star32,
    Star8,
    Star2,
}

impl Scrambler {
    fn apply(&self, state: u64) -> u32 {
        match self {
            Scrambler::None => (state >> 32) as u32,
            Scrambler::Plus => (state >> 32).wrapping_add(state << 32) as u32, // TODO: this seems obviously wrong
            Scrambler::Star32 => (state >> 32).wrapping_mul(M32) as u32,
            Scrambler::Star8 => (state >> 32).wrapping_mul(M8) as u32,
            Scrambler::Star2 => (state >> 32).wrapping_mul(M2) as u32,
        }
    }
}

pub struct Xorshift64 {
    state: u64,
    triple: (u64, u64, u64),
    rule: XorshiftRule,
    scrambler: Scrambler,
}

impl Xorshift64 {
    fn step(&mut self) {
        self.rule.apply(&mut self.state, self.triple);
    }
}

impl Iterator for Xorshift64 {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.scrambler.apply(self.state);
        self.step();
        Some(out)
    }
}
