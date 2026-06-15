pub mod blum_blum_shub;
pub mod lcg;
pub mod lfg;
pub mod lfsr;

pub use blum_blum_shub::*;
pub use lcg::*;
pub use lfg::*;
pub use lfsr::*;

// #[cfg(target_pointer_width = "32")]
// pub const UMAX: u32 = u32::MAX;
// #[cfg(target_pointer_width = "32")]
// pub const HALFUMAX: u32 = UMAX / 2;
// #[cfg(target_pointer_width = "32")]
// pub const SQRTUMAX: u32 = UMAX.isqrt();

// #[cfg(target_pointer_width = "64")]
// pub const UMAX: u64 = u64::MAX;
// #[cfg(target_pointer_width = "64")]

// pub const HALFUMAX: u64 = UMAX / 2;
// #[cfg(target_pointer_width = "64")]
// pub const SQRTUMAX: u64 = UMAX.isqrt();

pub const UMAX: u64 = u64::MAX;
pub const HALFUMAX: u64 = UMAX / 2;
pub const SQRTUMAX: u64 = UMAX.isqrt();
pub const UBITS: u64 = 64;
