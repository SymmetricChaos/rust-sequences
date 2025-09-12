// These sequences are core in the sense that any other sequence may depend on them but none of them can depend on others
pub mod arithmetic;
pub mod composite;
pub mod constant;
pub mod continued_fraction;
pub mod exponential;
pub mod factorial;
pub mod geometric;
pub mod integer;
pub mod natural;
pub mod nth_powers;
pub mod parity;
pub mod powers;
pub mod prime;
pub mod recurrence;
pub mod roots;
pub mod transforms;

// Easy imports from other places
pub use arithmetic::*;
pub use composite::*;
pub use constant::*;
pub use continued_fraction::*;
pub use exponential::*;
pub use factorial::*;
pub use geometric::*;
pub use integer::*;
pub use natural::*;
pub use nth_powers::*;
pub use parity::*;
pub use powers::*;
pub use prime::*;
pub use recurrence::*;
pub use roots::*;
pub use transforms::*;

use num::{BigInt, FromPrimitive};
use std::cell::LazyCell;
pub const TWO: LazyCell<BigInt> = LazyCell::new(|| BigInt::from_i32(2).unwrap());
pub const NEG_TWO: LazyCell<BigInt> = LazyCell::new(|| BigInt::from_i32(-2).unwrap());
