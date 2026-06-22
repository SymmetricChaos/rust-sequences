use crate::{Number, core::traits::Increment, utils::goodstein::hereditary_base_string};

/// Each positive natural in hereditary base notation used to define a Goodstein sequence.
///
/// ```test
/// The outermost terms are separated by spaces for ease of parsing.
/// n = 187, base = 3
/// 2*3^(3+1) + 2*3^2 + 2*3 + 1
///
/// base = 2
/// 1, 2, 2 + 1, 2^2, 2^2 + 1, 2^2 + 2, 2^2 + 2 + 1, 2^(2+1)...
///
/// base = 3
/// 1, 2, 3, 3 + 1, 3 + 2, 2*3, 2*3 + 1, 2*3 + 2, 3^2, 3^2 + 1, 3^2 + 2...
/// ```
pub struct HereditaryBaseStrings {
    ctr: Number,
    base: Number,
}

impl HereditaryBaseStrings {
    pub fn new(base: Number) -> Self {
        assert!(base >= 2);
        Self { ctr: 0, base }
    }
}

impl Iterator for HereditaryBaseStrings {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        Some(hereditary_base_string(self.ctr, self.base))
    }
}

crate::sample_sequences!(
    HereditaryBaseStrings::new(2);
    HereditaryBaseStrings::new(3);
);
