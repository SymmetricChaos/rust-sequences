use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, Integer};

use crate::{Increment, figurate::Cube};

pub struct Taxicab<T> {
    ctr: T,
    cubes: Vec<T>,
    cube: Cube<T>,
}

impl<T: CheckedAdd + CheckedMul + CheckedSub + Integer + Clone> Taxicab<T> {
    pub fn new() -> Self {
        let mut cube = Cube::<T>::new();
        cube.next();
        let mut cubes = Vec::new();
        cubes.push(cube.next().unwrap());
        Self {
            ctr: T::one(),
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

impl<T: CheckedAdd + CheckedMul + CheckedSub + Integer + Clone> Iterator for Taxicab<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            while &self.ctr >= self.cubes.last().unwrap() {
                self.cubes.push(self.cube.next()?);
            }
            let mut found_sum = false;
            for c in self.cubes.iter() {
                match self.ctr.checked_sub(c) {
                    Some(diff) => {
                        if self.cubes.contains(&diff) {
                            // check if we've found a sum of cubes before
                            if !found_sum {
                                found_sum = true;
                            } else {
                                // if we have found a sum before, check its not the same sum by making sure c is the smaller of the two terms
                                if &diff > c {
                                    return Some(self.ctr.clone());
                                }
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
    Taxicab::<i32>::new(), [10,20,40];
    Taxicab::new_big(), [10,20,40];
);

crate::check_sequences!(
    Taxicab::<i32>::new(), [1729, 4104, 13832, 20683, 32832, 39312, 40033, 46683, 64232, 65728, 110656, 110808, 134379, 149389, 165464, 171288, 195841, 216027, 216125, 262656, 314496, 320264, 327763, 373464, 402597, 439101, 443889, 513000, 513856, 515375, 525824, 558441, 593047, 684019, 704977];
);
