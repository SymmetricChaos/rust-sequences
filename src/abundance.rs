use crate::utils::divisibility::aliquot_sum;

/// The abundance of each positive integer.
pub struct Abundance {
    n: i64,
}

impl Abundance {
    pub fn new() -> Self {
        Self { n: 0 }
    }
}

impl Iterator for Abundance {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.n += 1;
        let x: i64 = aliquot_sum(self.n.try_into().unwrap())?
            .try_into()
            .expect("fail to convert");
        Some(x - self.n)
    }
}

crate::check_sequences!(
    Abundance::new(), 0, 15, [-1, -1, -2, -1, -4, 0, -6, -1, -5, -2, -10, 4, -12, -4, -6];
);
