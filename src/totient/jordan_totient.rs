use crate::{Number, core::traits::Increment, utils::totient::jordan_totient};

/// The Jordan totient of each positive integer.
///
/// ```text
/// k = 2
/// 1, 3, 8, 12, 24, 24, 48, 48, 72, 72, 120, 96, 168, 144, 192, 192...
///
/// k = 3
/// 1, 7, 26, 56, 124, 182, 342, 448, 702, 868, 1330, 1456, 2196, 2394...
/// ```
pub struct JordanTotients {
    ctr: Number,
    k: u32,
}

impl JordanTotients {
    pub fn new(k: u32) -> Self {
        Self { ctr: 0, k }
    }
}

impl Iterator for JordanTotients {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        self.ctr.incr()?;
        Some(jordan_totient(self.ctr, self.k))
    }
}

crate::check_sequences!(
    JordanTotients::new(2), [1, 3, 8, 12, 24, 24, 48, 48, 72, 72, 120, 96, 168, 144, 192, 192, 288, 216, 360, 288, 384, 360, 528, 384, 600, 504, 648, 576, 840, 576, 960, 768, 960, 864, 1152, 864, 1368, 1080, 1344, 1152, 1680, 1152, 1848, 1440, 1728, 1584, 2208, 1536];
    JordanTotients::new(3), [1, 7, 26, 56, 124, 182, 342, 448, 702, 868, 1330, 1456, 2196, 2394, 3224, 3584, 4912, 4914, 6858, 6944, 8892, 9310, 12166, 11648, 15500, 15372, 18954, 19152, 24388, 22568, 29790, 28672, 34580, 34384, 42408, 39312, 50652, 48006, 57096];
);

crate::sample_sequences!(
    JordanTotients::new(2);
    JordanTotients::new(3);
);
