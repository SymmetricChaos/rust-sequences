use crate::Number;
use num::Integer;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

/// Find a factor using Pollard's Rho. For inputs above above 67108863 parallelization is used.
pub fn pollards_rho(n: Number) -> Option<Number> {
    if n > 0x03FFFFFF {
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
            d = (x.abs_diff(y) as Number).gcd(&n);
        }
        if d != n {
            return Some(d);
        }
    }
    return None;
}

fn _pollards_rho_par(n: Number) -> Option<Number> {
    if n < 0xFFFFFFFF {
        (2..(n - 2)).into_par_iter().find_map_any(|s| {
            let mut x = s;
            let mut y = s;
            let mut d = 1_i64;
            while d == 1 {
                x = ((x * x) + 1) % n;
                y = ((y * y) + 1) % n;
                y = ((y * y) + 1) % n;
                d = (x.abs_diff(y) as Number).gcd(&n);
            }
            if d != n {
                return Some(d as Number);
            } else {
                return None;
            }
        })
    } else {
        let n = i128::from(n);
        (2..(n - 2)).into_par_iter().find_map_any(|s| {
            let mut x = s;
            let mut y = s;
            let mut d = 1;
            while d == 1 {
                x = ((x * x) + 1) % n;
                y = ((y * y) + 1) % n;
                y = ((y * y) + 1) % n;
                d = (x.abs_diff(y) as i128).gcd(&n);
            }
            if d != n { Some(d as Number) } else { None }
        })
    }
}
