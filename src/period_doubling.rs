use std::marker::PhantomData;

/// The period double sequence.
///
/// 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0...
pub struct PeriodDoubling<T> {
    _phantom: PhantomData<T>,
}

impl<T> PeriodDoubling<T> {
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Iterator for PeriodDoubling<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
