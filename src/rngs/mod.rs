pub mod blum_blum_shub;
pub mod lcg;
pub mod lfg;
pub mod lfsr;
pub mod mersenne_twister;
pub mod pcg;
pub mod rc4;
pub mod xorshift;

pub use blum_blum_shub::*;
pub use lcg::*;
pub use lfg::*;
pub use lfsr::*;
pub use mersenne_twister::*;
pub use pcg::*;
pub use rc4::*;
pub use xorshift::*;

pub type UNumber = u64;
pub const UMAX: UNumber = UNumber::MAX;
pub const HALFUMAX: UNumber = UMAX / 2;
pub const SQRTUMAX: UNumber = UMAX.isqrt();
pub const UBITS: UNumber = 64;
