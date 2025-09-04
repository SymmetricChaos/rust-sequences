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

pub use arithmetic::Arithmetic;
pub use composite::Composite;
pub use constant::Constant;
pub use geometric::Geometric;
pub use integer::Integer;
pub use natural::{Counting, Natural};
pub use parity::{Even, EvenInteger, Odd, OddInteger};
pub use power::Power;
pub use prime::Prime;
