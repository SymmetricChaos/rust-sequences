use crate::Number;
use num::Integer;

pub fn factorable(n: Number, factors: &[Number]) -> bool {
    if factors.contains(&0) {
        panic!("factors cannot contain zero")
    }
    if factors.contains(&1) {
        panic!("factors cannot contain one")
    }
    factorable_inner(n, factors, 0)
}

/// Can n be factored using only the numbers in factors?
pub fn factorable_inner(n: Number, factors: &[Number], level: usize) -> bool {
    if n == 1 {
        return true;
    }
    if level == factors.len() {
        return false;
    }
    let (q, r) = n.div_rem(&factors[level]);
    if r == 0 {
        return factorable_inner(q, factors, level) || factorable_inner(n, factors, level + 1);
    } else {
        return factorable_inner(n, factors, level + 1);
    }
}

#[cfg(test)]
#[test]
fn factor() {
    println!("{}", factorable_inner(72, &[2, 6, 24], 0))
}
