use num::Integer;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// Find a factor using Pollard's Rho.
/// Uses 64-bit arithmetic on a single thread for small numbers.
/// Uses parallelized 64-bit arithmetic up to 2^32 and parallelized 128-bit arithmetic for larger numbers.
pub fn pollards_rho(n: u64) -> Option<u64> {
    if n > 0x3FFFFFF {
        return _pollards_rho_par(n);
    }

    for s in 2..(n - 2) {
        let mut x = s;
        let mut y = s;
        let mut d = 1;
        while d == 1 {
            x = ((x * x) + 1) % n;
            y = ((y * y) + 1) % n;
            y = ((y * y) + 1) % n;
            d = x.abs_diff(y).gcd(&n);
        }
        if d != n {
            return Some(d);
        }
    }
    return None;
}

fn _pollards_rho_par(n: u64) -> Option<u64> {
    if n < 0xFFFFFFFF {
        (2..(n - 2)).into_par_iter().find_map_any(|s| {
            let mut x = s;
            let mut y = s;
            let mut d = 1;
            while d == 1 {
                x = ((x * x) + 1) % n;
                y = ((y * y) + 1) % n;
                y = ((y * y) + 1) % n;
                d = x.abs_diff(y).gcd(&n);
            }
            if d != n {
                return Some(d as u64);
            } else {
                return None;
            }
        })
    } else {
        let n = u128::from(n);
        (2..(n - 2)).into_par_iter().find_map_any(|s| {
            let mut x = s;
            let mut y = s;
            let mut d = 1;
            while d == 1 {
                x = ((x * x) + 1) % n;
                y = ((y * y) + 1) % n;
                y = ((y * y) + 1) % n;
                d = x.abs_diff(y).gcd(&n);
            }
            if d != n { Some(d as u64) } else { None }
        })
    }
}
