use crate::Number;
use num::Integer;

/// Can n be factored using the number numbers in factors?
pub fn factorable(n: Number, factors: &[Number], level: usize) -> bool {
    if n == 1 {
        return true;
    }
    if level == factors.len() {
        return false;
    }
    let (q, r) = n.div_rem(&factors[level]);
    if r == 0 {
        return factorable(q, factors, level) || factorable(n, factors, level + 1);
    } else {
        return factorable(n, factors, level + 1);
    }
}

#[cfg(test)]
#[test]
fn factor() {
    println!("{}", factorable(72, &[2, 6, 24], 0))
}
