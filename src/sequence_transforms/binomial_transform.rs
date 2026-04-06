use crate::core::alternating::Alternating;
use num::{CheckedAdd, CheckedMul, CheckedSub, Integer, Signed, integer::binomial};

pub struct BinomialTransform<T> {
    iter: Box<dyn Iterator<Item = T>>,
    saved: Vec<T>,
    n: T,
}

impl<T: Clone + Integer + Signed + CheckedAdd + CheckedSub + CheckedMul> BinomialTransform<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            iter: Box::new(iter),
            saved: Vec::new(),
            n: T::zero(),
        }
    }
}

impl<T: Clone + Integer + Signed + CheckedAdd + CheckedSub + CheckedMul> Iterator
    for BinomialTransform<T>
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.saved.push(self.iter.next()?);
        let mut k = T::zero();
        let mut out = T::zero();

        for (term, add) in self.saved.iter().zip(Alternating::true_false()) {
            let b = binomial(self.n.clone(), k.clone());
            let pr = term.checked_mul(&b)?;
            if add {
                out = out.checked_add(&pr)?;
            } else {
                out = out.checked_sub(&pr)?;
            }
            k = k + T::one();
        }
        self.n = self.n.checked_add(&T::one())?;

        Some(out)
    }
}

crate::check_sequences!(
    BinomialTransform::new([0, 1, 10, 63, 324, 1485].into_iter()), [0, -1, 8, -36, 128, -400];
    BinomialTransform::new([0, -1, 8, -36, 128, -400].into_iter()), [0, 1, 10, 63, 324, 1485]; // notice that the transform is an involution
);
