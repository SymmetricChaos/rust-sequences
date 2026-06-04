use crate::figurate::Cube;
use crate::{Number, core::traits::Increment};
use num::BigInt;

/// The taxicab numbers. The natural numbers that are the sum of two positive cubes in more than one way. Named for an ancedote by G. H. Hardy about Srinivasa Ramanujan.
///
/// ```text
/// 1729, 4104, 13832, 20683, 32832, 39312, 40033, 46683...
/// ```
pub struct Taxicab<T> {
    ctr: T,
    cubes: Vec<T>,
    cube: Cube<T>,
}

impl Taxicab<Number> {
    pub fn new() -> Self {
        let mut cube = Cube::new();
        cube.next();
        let mut cubes = Vec::new();
        for _ in 0..12 {
            cubes.push(cube.next().unwrap());
        }
        Self {
            ctr: 1728,
            cubes,
            cube,
        }
    }
}

impl Taxicab<BigInt> {
    pub fn new_big() -> Self {
        let mut cube = Cube::new_big();
        cube.next();
        let mut cubes = Vec::new();
        for _ in 0..12 {
            cubes.push(cube.next().unwrap());
        }
        Self {
            ctr: BigInt::from(1728),
            cubes,
            cube,
        }
    }
}

impl Iterator for Taxicab<Number> {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            if &self.ctr >= self.cubes.last().unwrap() {
                self.cubes.push(self.cube.next()?);
            }
            let mut found_sum = false;
            for c in self.cubes.iter() {
                match self.ctr.checked_sub(*c) {
                    Some(diff) => {
                        if self.cubes.contains(&diff) {
                            // check if we've found a sum of cubes before
                            if found_sum {
                                // if we have found a sum before, check its not the same sum in reverse order by making sure c is the smaller of the two terms
                                if &diff > c {
                                    return Some(self.ctr);
                                }
                            } else {
                                // otherwise note that we've found a sum
                                found_sum = true;
                            }
                        }
                    }
                    None => continue,
                }
            }
        }
    }
}

impl Iterator for Taxicab<BigInt> {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            if &self.ctr >= self.cubes.last().unwrap() {
                self.cubes.push(self.cube.next()?);
            }
            let mut found_sum = false;
            for c in self.cubes.iter() {
                match self.ctr.checked_sub(c) {
                    Some(diff) => {
                        if self.cubes.contains(&diff) {
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
                    None => continue,
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
