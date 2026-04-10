#[cfg(test)]
use crate::{
    core::{
        constant::Constant,
        continued_fraction::SimpleContinuedFraction,
        exponential::Exponential,
        prime::Primes,
        rational_digits::rational_decimal_string as to_dec,
        roots::{CubeRoot, SquareRoot},
    },
    dirichlet_convolution::{DirichletConvolution, one},
    fibonacci::Fibonacci,
    figurate::Triangular,
    harmonic::{Harmonic, HarmonicNumbers},
    ruler::Ruler,
    sequence_transforms::{
        boustrophedon::{Boustrophedon, BoustrophedonTriangle},
        differences::AbsDiffs,
        lodumo::Lodumo,
        ordinal::OrdinalTransform,
        rational::Ratios,
        sums::{AlternatingPartialSums, CesaroPartialSums, PartialSums},
    },
    zeta::Zeta,
};

crate::print_sequences!(
    print_integers;
    Primes::new_big(), 10;
    Primes::new_big(), skip 5, 10;
    Primes::new_big(), 10, "{:>3}", "|";
    Primes::new_big(), skip 5, 10, "{:>3}", "|";
    PartialSums::new(Harmonic::new_big()), 10;
    AlternatingPartialSums::new(Harmonic::new_big()), 10;
    HarmonicNumbers::<i64>::new(), 10;
    Ratios::new(Primes::new_big(),Fibonacci::new_big().skip(1)), 10;
    AbsDiffs::new(Primes::new_big()), 10;
    Triangular::new_big(), skip  1234567, 5; // show fast skip ahead
    Boustrophedon::new(Constant::new(1)), 10;
    Exponential::new_big(1,1), 8; // converges on e
    Zeta::new_big(3), 9; // converges on Apery's constant 1.202569...
    SimpleContinuedFraction::new(Constant::new(1)).map(|q| to_dec(q, 5).unwrap()), 10; // converges on golden ratio 1.61803398875...
    SimpleContinuedFraction::new(Constant::new(2)).map(|q| to_dec(q, 5).unwrap()), 10; // converges on silver ratio 2.414213562...
    SimpleContinuedFraction::new_periodic(&[1], &[2]).map(|q| to_dec(q, 5).unwrap()), 10; // converges on sqrt(2) 1.41421356237...
    SquareRoot::new_big(1,2).map(|q| to_dec(q, 5).unwrap()), 6; // converges on sqrt(1/2) 0.70710678118...
    CubeRoot::new_big(2,1).map(|q| to_dec(q, 5).unwrap()), 6; // converges on cbrt(2) 1.25992104989...
    CesaroPartialSums::new([1,0].into_iter().cycle()), 10;
    DirichletConvolution::new_big(one,one), 10; // identical to number of divisors function
    BoustrophedonTriangle::new(Primes::new_big()), 5, "{:?}", "\n";
    Ruler::<u32>::new(), 20;
    OrdinalTransform::new(Ruler::<u32>::new()), 20;
    Lodumo::new(Fibonacci::<u32>::new(),5), 20; // A160081
);
