#[cfg(test)]
use crate::{
    core::{
        natural::Natural,
        prime::Prime,
        transforms::{AbsDiffs, PartialSums, Ratios},
    },
    fibonacci::Fibonacci,
    harmonic::Harmonic,
};

crate::print_a_few!(
    Natural::from(1), 0, 10;
    PartialSums::new(Natural::from(1)), 0, 10;
    PartialSums::new(Harmonic::new()), 0, 10;
    Ratios::new(Prime::new(),Fibonacci::new().skip(1)), 0, 10;
    AbsDiffs::new(Prime::new()), 0, 10;
);
