use std::ops::Rem;

use num::{CheckedAdd, Integer};

use crate::core::traits::Increment;

/// The shadow transform.
pub struct ShadowTransform<T> {
    iter: Box<dyn Iterator<Item = T>>,
    terms: Vec<T>,
    ctr: T,
}

impl<T: CheckedAdd + Clone + Integer> ShadowTransform<T> {
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            iter: Box::new(iter),
            terms: Vec::new(),
            ctr: T::zero(),
        }
    }
}

impl<T: CheckedAdd + Clone + Integer + Rem<Output = T>> Iterator for ShadowTransform<T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut total = 0;

        for i in self.terms.iter() {
            if (i.clone() % self.ctr.clone()).is_zero() {
                total += 1;
            }
        }

        self.terms.push(self.iter.next()?);
        self.ctr.incr()?;

        Some(total)
    }
}

crate::check_sequences!(
    ShadowTransform::new(
        [
            1_u64,
            2,
            5,
            16,
            65,
            326,
            1957,
            13700,
            109601,
            986410,
            9864101,
            108505112,
            1302061345,
            16926797486,
            236975164805,
            3554627472076,
            56874039553217,
            966858672404690,
            17403456103284421,
            330665665962404000,
            6613313319248080001
        ]
        .into_iter()
    ),
    [
        0, 1, 1, 0, 1, 2, 0, 0, 1, 0, 2, 0, 0, 3, 0, 0, 1, 0, 0, 2, 2
    ];
);
