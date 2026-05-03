//! These sequences are core in the sense that any other sequence may depend on them but none of them can depend on any others.
//! They should be as generic as possible and avoid using supertraits like Integer or Num that have many subtraits.
pub mod arithmetic;
pub mod bernoulli_numbers;
pub mod catalan;
pub mod composite;
pub mod constant;
pub mod geometric;
pub mod integer;
pub mod natural;
pub mod nth_powers;
pub mod parity;
pub mod polynomial;
pub mod powers;
pub mod prime_omega;
pub mod primes;
pub mod rational_digits;
pub mod rationals;
pub mod recurrence;
pub mod roots;

pub mod alternating;
pub mod combinations;
pub mod continued_fraction;
pub mod traits;

pub use alternating::*;
pub use arithmetic::*;
pub use bernoulli_numbers::*;
pub use catalan::*;
pub use combinations::*;
pub use composite::*;
pub use constant::*;
pub use continued_fraction::*;
pub use geometric::*;
pub use integer::*;
pub use natural::*;
pub use nth_powers::*;
pub use parity::*;
pub use polynomial::*;
pub use powers::*;
pub use prime_omega::*;
pub use primes::*;
pub use rational_digits::*;
pub use rationals::*;
pub use recurrence::*;
pub use roots::*;
