#[cfg(test)]
use crate::{naturals::Natural, transforms::PartialSums};

crate::print_a_few!(
    Natural::new().skip(1), 0, 10;
    PartialSums::new(Natural::new().skip(1)), 0, 10;
);
