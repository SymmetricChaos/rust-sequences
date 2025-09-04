#[cfg(test)]
use crate::{
    core::{
        natural::{Counting, Natural},
        prime::Prime,
    },
    fibonacci::Fibonacci,
    harmonic::Harmonic,
    transforms::{AbsDiffs, PartialSums, Ratios},
};

crate::print_a_few!(
    Natural::new().skip(1), 0, 10;
    PartialSums::new(Counting::new()), 0, 10;
    PartialSums::new(Harmonic::new()), 0, 10;
    Ratios::new(Prime::new(),Fibonacci::new().skip(1)), 0, 10;
    AbsDiffs::new(Prime::new()), 0, 10;
);
