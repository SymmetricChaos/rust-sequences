// These sequences are core in the sense that any other sequence may depend on them but none of them can depend on others
pub mod arithmetic;
pub mod composite;
pub mod constant;
pub mod exponent;
pub mod exponential;
pub mod factorial;
pub mod geometric;
pub mod integer;
pub mod natural;
pub mod parity;
pub mod powers;
pub mod prime;
pub mod transforms;

// Easy imports from other places
pub use arithmetic::*;
pub use composite::*;
pub use constant::*;
pub use exponent::*;
pub use exponential::*;
pub use factorial::*;
pub use geometric::*;
pub use integer::*;
pub use natural::*;
pub use parity::*;
pub use powers::*;
pub use prime::*;
pub use transforms::*;

use num::{BigInt, FromPrimitive};
use std::cell::LazyCell;
pub const TWO: LazyCell<BigInt> = LazyCell::new(|| BigInt::from_i32(2).unwrap());
pub const NEG_TWO: LazyCell<BigInt> = LazyCell::new(|| BigInt::from_i32(-2).unwrap());
pub const NEG_ONE: LazyCell<BigInt> = LazyCell::new(|| BigInt::from_i32(-1).unwrap());
