use crate::figurate::Cube;
use crate::{Number, core::traits::Increment};
use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, Integer};

/// The taxicab numbers. The natural numbers that are the sum of two positive cubes in more than one way. Named for an ancedote by G. H. Hardy about Srinivasa Ramanujan.
///
/// ```text
/// 1729, 4104, 13832, 20683, 32832, 39312, 40033, 46683, 64232, 65728...
/// ```
pub struct Taxicab<T> {
    ctr: T,
    cube_list: Vec<T>,
    cubes: Cube<T>,
}

impl Taxicab<Number> {
    pub fn new() -> Self {
        let mut cubes = Cube::new();
        cubes.next();
        let mut cube_list = Vec::new();
        // Preload the list of cubes because we known the first taxicab number is 1729.
        for _ in 0..12 {
            cube_list.push(cubes.next().unwrap());
        }
        Self {
            ctr: 1728,
            cube_list,
            cubes,
        }
    }
}

#[cfg(feature = "big_int")]
impl Taxicab<BigInt> {
    pub fn new_big() -> Self {
        let mut cubes = Cube::new_big();
        cubes.next();
        let mut cube_list = Vec::new();
        for _ in 0..12 {
            cube_list.push(cubes.next().unwrap());
        }
        Self {
            ctr: BigInt::from(1728),
            cube_list,
            cubes,
        }
    }
}

impl<T: Clone + CheckedMul + CheckedSub + CheckedAdd + Integer> Iterator for Taxicab<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            if &self.ctr >= self.cube_list.last().unwrap() {
                self.cube_list.push(self.cubes.next()?);
            }
            let mut found_sum = false;
            for c in self.cube_list.iter() {
                match self.ctr.checked_sub(c) {
                    Some(diff) => {
                        if self.cube_list.contains(&diff) {
                            // check if we've found a sum of cubes before
                            if found_sum {
                                // if we have found a sum before, check its not the same sum in reverse order by making sure c is the smaller of the two terms
                                if &diff > c {
                                    return Some(self.ctr.clone());
                                }
                            } else {
                                // otherwise note that we've found a sum
                                found_sum = true;
                            }
                        }
                    }
                    None => continue, // this only happens for unsigned types, signed types will produce at most one negative difference so there's no reason to optimize it away
                }
            }
        }
    }
}

crate::check_iteration_times!(
    Taxicab::new(), [10,20,30];
    Taxicab::new_big(), [10,20,30];
);

crate::check_sequences!(
    Taxicab::new(), [1729, 4104, 13832, 20683, 32832, 39312, 40033, 46683, 64232, 65728, 110656, 110808, 134379, 149389, 165464, 171288, 195841, 216027, 216125, 262656, 314496, 320264, 327763, 373464, 402597, 439101, 443889, 513000, 513856, 515375, 525824, 558441, 593047, 684019, 704977];
    Taxicab::new_big(), [1729, 4104, 13832, 20683, 32832, 39312, 40033, 46683, 64232, 65728, 110656, 110808, 134379, 149389, 165464, 171288, 195841, 216027, 216125, 262656, 314496, 320264, 327763, 373464, 402597, 439101, 443889, 513000, 513856, 515375, 525824, 558441, 593047, 684019, 704977];
);

crate::sample_sequences!(
    Taxicab::new();
);
