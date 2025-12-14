/// Results true if the ath bit of b is 1.
pub fn bit_predicate<T>(a: T, b: T) -> bool
where
    u32: From<T>,
{
    if let Some(bit) = u32::from(b).checked_shr(u32::from(a)) {
        (bit & 1) == 1
    } else {
        false
    }
}
