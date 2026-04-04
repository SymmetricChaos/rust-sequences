use std::marker::PhantomData;

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
