#[cfg(test)]
use crate::{
    fibonacci::Fibonacci,
    harmonic::Harmonic,
    naturals::{Counting, Natural},
    primes::Prime,
    transforms::{AbsDiffs, PartialSums, Ratios},
};

crate::print_a_few!(
    Natural::new().skip(1), 0, 10;
    PartialSums::new(Counting::new()), 0, 10;
    PartialSums::new(Harmonic::new()), 0, 10;
    Ratios::new(Prime::new(),Fibonacci::new().skip(1)), 0, 10;
    AbsDiffs::new(Prime::new()), 0, 10;
);
