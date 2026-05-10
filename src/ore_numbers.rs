use num::rational::Ratio;

use crate::{
    core::traits::Increment,
    utils::divisibility::{number_of_divisors, sum_of_divisors},
};

pub struct Ore {
    ctr: u64,
}

impl Ore {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Ore {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.ctr.incr()?;
            let s0 = number_of_divisors(self.ctr);
            let s1 = sum_of_divisors(self.ctr)?;
            if Ratio::new(s0.checked_mul(self.ctr)?, s1).is_integer() {
                return Some(self.ctr);
            }
        }
    }
}

crate::check_sequences!(
    Ore::new(), [1, 6, 28, 140, 270, 496, 672, 1638, 2970, 6200, 8128, 8190, 18600, 18620, 27846, 30240, 32760, 55860, 105664, 117800, 167400, 173600, 237510, 242060, 332640, 360360, 539400, 695520, 726180, 753480, 950976, 1089270, 1421280, 1539720];
);
