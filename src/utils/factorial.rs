use num::Integer;

pub fn factorial<T: Integer + Clone>(mut n: T) -> T {
    let mut out = n.clone();
    while n > T::one() {
        n = n - T::one();
        out = out * n.clone();
    }
    out
}
