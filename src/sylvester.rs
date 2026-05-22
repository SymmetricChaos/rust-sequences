use num::{BigInt, CheckedAdd, CheckedMul, CheckedSub, One};

/// Sylvester's sequence. Starting at 2 each term is the product of all previous terms plus one. Overflow occurs quickly for fixed width integer types.
///
/// 2, 3, 7, 43, 1807, 3263443...
pub struct Sylvester<T> {
    current: T,
    overflowed: bool,
}

impl<T: Clone + One + CheckedMul + CheckedAdd + CheckedSub> Sylvester<T> {
    pub fn new() -> Self {
        Self {
            current: T::one() + T::one(),
            overflowed: false,
        }
    }
}

impl Sylvester<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + One + CheckedMul + CheckedAdd + CheckedSub> Iterator for Sylvester<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.current.clone();

        // Always returns some but this avoids a clone.
        let m1 = self.current.checked_sub(&T::one()).unwrap();

        self.current = match self.current.checked_mul(&m1) {
            Some(n) => match n.checked_add(&T::one()) {
                Some(n) => n,
                None => {
                    self.overflowed = true;
                    return Some(out);
                }
            },
            None => {
                self.overflowed = true;
                return Some(out);
            }
        };

        Some(out)
    }
}

crate::check_sequences!(
    Sylvester::<u64>::new(), [2_u64, 3, 7, 43, 1807, 3263443, 10650056950807]; // check that maximum value gets returned
    Sylvester::new_big(), ["2", "3", "7", "43", "1807", "3263443", "10650056950807", "113423713055421844361000443", "12864938683278671740537145998360961546653259485195807", "165506647324519964198468195444439180017513152706377497841851388766535868639572406808911988131737645185443"];
);
