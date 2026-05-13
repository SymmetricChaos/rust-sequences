use crate::{
    core::{Primes, traits::Increment},
    utils::{divisibility::prime_factorization, miller_rabin::is_prime},
};
use std::collections::VecDeque;

/// The Blum integers. Natural numbers of the form p*q where p and q are primes congruent to 3 modulo 4 and p is not equal to q. They are relevant to the Blum-Blum-Shub PRNG.
///
/// 21, 33, 57, 69, 77, 93, 129, 133, 141...
pub struct Blum {
    ctr: u64,
}

impl Blum {
    pub fn new() -> Self {
        Self { ctr: 20 }
    }
}

impl Iterator for Blum {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            let f = prime_factorization(self.ctr);
            if f.len() == 2 && f[0].1 == 1 && f[1].1 == 1 {
                if f[0].0 % 4 == 3 && f[1].0 % 4 == 3 {
                    return Some(self.ctr);
                }
            }
        }
    }
}

/// Primes that can be factors of the modulus of a maximum length Blue-Blum-Shub PRNG.
///
/// 23, 47, 167, 359, 719, 1439, 2039...
pub struct BlumBlumShubPrimes {
    primes: Primes<u64>,
}

impl BlumBlumShubPrimes {
    pub fn new() -> Self {
        Self {
            primes: Primes::new(),
        }
    }
}

impl Iterator for BlumBlumShubPrimes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let p = self.primes.next()?;
            if p % 4 != 3 {
                continue;
            }
            let a = (p - 1) / 2;
            let b = (a - 1) / 2;
            if a == 2 || b == 2 {
                continue;
            }
            if is_prime(a) && is_prime(b) {
                return Some(p);
            }
        }
    }
}

/// The values of a modulus that give maximum period for the Blum-Blum-Shub PRNG.
///
/// 1081, 3841, 7849, 8257, 16537, 16873, 33097, 46897...
pub struct BlumBlumShubMaximum {
    bbsp: BlumBlumShubPrimes,
    s: Vec<u64>,
    t: VecDeque<u64>,
    p: u64,
}

impl BlumBlumShubMaximum {
    pub fn new() -> Self {
        Self {
            bbsp: BlumBlumShubPrimes::new(),
            s: Vec::new(),
            t: VecDeque::new(),
            p: 0,
        }
    }
}

impl Iterator for BlumBlumShubMaximum {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.t.len() > 0 && self.t[0] < self.p * self.s[0] {
                return self.t.pop_front();
            } else {
                let p = self.bbsp.next()?;
                for s in self.s.iter() {
                    // is 2 a quadratic residue (mod p/2) or (mod s/2), but not both
                    // it is only necessary to check that they are equal to 7 (mod 8) as all other conditions have been ruled out
                    if !(((p - 1) / 2 % 8 == 7) && ((s - 1) / 2 % 8 == 7)) {
                        self.t.push_back(p * s);
                    }
                }
                self.p = p;
                self.s.push(p);
                self.t.make_contiguous().sort();
            }
        }
    }
}

/// Sequence of state for a Blum-Blum-Shub PRNG. These states are not returned directly by the actual algorithm, however, instead a few bits being extracted via parity or masking.
pub struct BlumBlumShub {
    val: u128,
    modulus: u128,
}

impl BlumBlumShub {
    pub fn new(inital_value: u64, modulus: u64) -> Self {
        Self {
            val: inital_value as u128,
            modulus: modulus as u128,
        }
    }
}

impl Iterator for BlumBlumShub {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val as u64;
        self.val = (self.val * self.val) % self.modulus;
        Some(out)
    }
}

crate::check_sequences!(
    Blum::new(), [21, 33, 57, 69, 77, 93, 129, 133, 141, 161, 177, 201, 209, 213, 217, 237, 249, 253, 301, 309, 321, 329, 341, 381, 393, 413, 417, 437, 453, 469, 473, 489, 497, 501, 517, 537, 553, 573, 581, 589, 597, 633, 649, 669, 681, 713, 717, 721, 737, 749, 753, 781, 789];
    BlumBlumShubPrimes::new(), [23, 47, 167, 359, 719, 1439, 2039, 2879, 4079, 4127, 4919, 5639, 5807, 5927, 6047, 7247, 7559, 7607, 7727, 9839, 10799, 11279, 13799, 13967, 14159, 15287, 15647, 20327, 21599, 21767, 23399, 24407, 24527, 25799, 28319, 28607, 29399];
    BlumBlumShubMaximum::new(), [1081, 3841, 7849, 8257, 16537, 16873, 33097, 46897, 59953, 66217, 93817, 94921, 95833, 113137, 120073, 129697, 133561, 136321, 139081, 166681, 173857, 174961, 177721, 226297, 231193, 240313, 248377, 258121, 259417, 265033, 278569, 317377, 321241, 325657];
);
