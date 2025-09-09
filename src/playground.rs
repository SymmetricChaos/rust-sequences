#[cfg(test)]
use crate::{
    core::{
        natural::Natural,
        prime::Prime,
        transforms::{AbsDiffs, PartialSums, Ratios},
    },
    fibonacci::Fibonacci,
    figurate::Triangular,
    harmonic::Harmonic,
};

crate::print_values!(
    Natural::from(1), 0, 10;
    PartialSums::new(Natural::from(1)), 0, 10;
    PartialSums::new(Harmonic::new()), 0, 10;
    Ratios::new(Prime::new(),Fibonacci::new().skip(1)), 0, 10;
    AbsDiffs::new(Prime::new()), 0, 10;
    Natural::new().skip(5), 0, 10;
    Triangular::new().skip(5000), 0, 5;
);
