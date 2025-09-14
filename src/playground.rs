#[cfg(test)]
use crate::{
    core::{
        AbsDiffs, Boustrophedon, BoustrophedonTriangle, Constant, CubeRoot, Exponential,
        PartialSums, Prime, Ratios, SimpleContinuedFraction, SquareRoot,
    },
    fibonacci::Fibonacci,
    figurate::Triangular,
    harmonic::{Harmonic, HarmonicSums},
    zeta::Zeta,
};
#[cfg(test)]
use num::BigInt;

crate::print_values!(
    print_integers, formatter "{}", sep ", ";
    PartialSums::new(Harmonic::new()), 0, 10;
    HarmonicSums::new(), 0, 10;
    Ratios::new(Prime::new(),Fibonacci::new().skip(1)), 0, 10;
    AbsDiffs::new(Prime::new()), 0, 10;
    Triangular::new().skip(1_234_567), 0, 5; // show fast skip ahead
    Boustrophedon::new(Constant::new(1)), 0, 10;
    Exponential::new(1,1), 0, 8; // converges on e
    Zeta::new(3), 0, 9; // converges on Apery's constant 1.202569...
    SimpleContinuedFraction::new(Constant::new(1)), 0, 10; // converges on golden ratio 1.61803398875...
    SimpleContinuedFraction::new(Constant::new(2)), 0, 10; // converges on silver ratio 2.414213562...
    SimpleContinuedFraction::new_periodic(crate::big!(vec![1]), crate::big!(vec![2])), 0, 10; // converges on sqrt(2) 1.41421356237...
    SquareRoot::new(1,2), 0, 6; // converges on sqrt(1/2) 0.70710678118...
    CubeRoot::new(2,1), 0, 5; // converges on cbrt(2) 1.25992104989...
);

crate::print_values!(
    print_triangles, formatter "{:?}", sep "\n";
    BoustrophedonTriangle::new(Prime::new()), 0, 5;
);
