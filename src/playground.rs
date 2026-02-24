#[cfg(test)]
use crate::{
    core::{
        boustrophedon::{Boustrophedon, BoustrophedonTriangle},
        constant::Constant,
        continued_fraction::SimpleContinuedFraction,
        differences::AbsDiffs,
        dirichlet_convolution::{DirichletConvolution, one},
        exponential::Exponential,
        prime::Primes,
        rational_digits::rational_decimal_string as dec,
        rational_transforms::Ratios,
        roots::{CubeRoot, SquareRoot},
        summation::{CesaroPartialSums, PartialSums},
    },
    fibonacci::Fibonacci,
    figurate::Triangular,
    harmonic::{Harmonic, HarmonicNumbers},
    zeta::Zeta,
};

crate::print_sequences!(
    print_integers;
    PartialSums::new(Harmonic::new_big()), 10;
    HarmonicNumbers::<i64>::new(), 10;
    Ratios::new(Primes::new_big(),Fibonacci::new_big().skip(1)), 10;
    AbsDiffs::new(Primes::new_big()), 10;
    Triangular::new_big(), 1234567, 5; // show fast skip ahead
    Boustrophedon::new(Constant::new(1)), 10;
    Exponential::new_big(1,1), 8; // converges on e
    Zeta::new_big(3), 9; // converges on Apery's constant 1.202569...
    SimpleContinuedFraction::new(Constant::new(1)), 10; // converges on golden ratio 1.61803398875...
    SimpleContinuedFraction::new(Constant::new(2)), 10; // converges on silver ratio 2.414213562...
    SimpleContinuedFraction::new_periodic(&[1], &[2]), 10; // converges on sqrt(2) 1.41421356237...
    SimpleContinuedFraction::new_periodic(&[1], &[2]).map(|q| dec(q, 5).unwrap()), 10; // converges on sqrt(2) 1.41421356237...
    SquareRoot::new_big(1,2), 6; // converges on sqrt(1/2) 0.70710678118...
    CubeRoot::<u64>::new(2,1), 5; // converges on cbrt(2) 1.25992104989...
    CesaroPartialSums::new([1,0].into_iter().cycle()), 10;
    DirichletConvolution::new_big(one,one), 10; // identical to number of divisors function
    BoustrophedonTriangle::new(Primes::new_big()), 5, "{:?}", "\n";
);
