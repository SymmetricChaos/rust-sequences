pub mod blum_blum_shub;
pub mod lcg;
pub mod lfg;
pub mod lfsr;

pub use blum_blum_shub::*;
pub use lcg::*;
pub use lfg::*;
pub use lfsr::*;

pub const UMAX: u64 = u64::MAX;
pub const HALFUMAX: u64 = UMAX / 2;
pub const SQRTUMAX: u64 = UMAX.isqrt();
pub const UBITS: u64 = 64;
