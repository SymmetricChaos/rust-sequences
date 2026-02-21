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
        rational_transforms::Ratios,
        roots::{CubeRoot, SquareRoot},
        summation::{CesaroPartialSums, PartialSums},
    },
    fibonacci::Fibonacci,
    figurate::Triangular,
    harmonic::{Harmonic, HarmonicSums},
    zeta::Zeta,
};
#[cfg(test)]
use num::BigInt;

crate::print_sequences!(
    print_integers, formatter "{}", sep ", ";
    PartialSums::new(Harmonic::new_big()), 0, 10;
    HarmonicSums::new_big(), 0, 10;
    Ratios::new(Primes::new_big(),Fibonacci::new_big().skip(1)), 0, 10;
    AbsDiffs::new(Primes::new_big()), 0, 10;
    Triangular::new_big().skip(1_234_567), 0, 5; // show fast skip ahead
    Boustrophedon::new(Constant::new(1)), 0, 10;
    Exponential::new_big(1,1), 0, 8; // converges on e
    Zeta::new_big(3), 0, 9; // converges on Apery's constant 1.202569...
    SimpleContinuedFraction::new(Constant::new_big(1)), 0, 10; // converges on golden ratio 1.61803398875...
    SimpleContinuedFraction::new(Constant::new_big(2)), 0, 10; // converges on silver ratio 2.414213562...
    SimpleContinuedFraction::new_periodic(crate::big!(vec![1]), crate::big!(vec![2])), 0, 10; // converges on sqrt(2) 1.41421356237...
    SquareRoot::new_big(1,2), 0, 6; // converges on sqrt(1/2) 0.70710678118...
    CubeRoot::<u64>::new(2,1), 0, 5; // converges on cbrt(2) 1.25992104989...
    CesaroPartialSums::new([1,0].into_iter().cycle()), 0, 10;
    DirichletConvolution::new_big(one,one), 0, 10; // identical to number of divisors function
);

crate::print_sequences!(
    print_triangles, formatter "{:?}", sep "\n";
    BoustrophedonTriangle::new(Primes::new_big()), 0, 5;
);
