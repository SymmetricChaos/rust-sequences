#[cfg(test)]
use crate::{harmonic::Harmonic, naturals::Natural, transforms::PartialSums};

crate::print_a_few!(
    Natural::new().skip(1), 0, 10;
    PartialSums::new(Natural::new().skip(1)), 0, 10;
    PartialSums::new(Harmonic::new()), 0, 10;
);
