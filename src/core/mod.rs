// These sequences are core in the sense that any other sequence may depend on them but none of them can depend on others
pub mod arithmetic;
pub mod composite;
pub mod constant;
pub mod geometric;
pub mod integer;
pub mod natural;
pub mod parity;
pub mod power;
pub mod prime;
pub mod transforms;

pub use arithmetic::Arithmetic;
pub use composite::Composite;
pub use constant::Constant;
pub use geometric::Geometric;
pub use integer::Integers;
pub use natural::Natural;
use num::{BigInt, FromPrimitive};
pub use parity::{Even, EvenInteger, Odd, OddInteger};
pub use power::Power;
pub use prime::Prime;

use std::cell::LazyCell;

pub const TWO: LazyCell<BigInt> = LazyCell::new(|| BigInt::from_i32(2).unwrap());
pub const NEG_TWO: LazyCell<BigInt> = LazyCell::new(|| BigInt::from_i32(-2).unwrap());
pub const NEG_ONE: LazyCell<BigInt> = LazyCell::new(|| BigInt::from_i32(-1).unwrap());
