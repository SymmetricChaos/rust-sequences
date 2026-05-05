pub struct Records<T, G> {
    iter: Box<dyn Iterator<Item = T>>,
    record: G,
    compare: Box<dyn Fn(&T) -> G>,
}

impl<T, G> Records<T, G> {
    pub fn new<I, C>(iter: I, compare: C, start: G) -> Self
    where
        I: Iterator<Item = T> + 'static,
        C: Fn(&T) -> G + 'static,
    {
        Self {
            iter: Box::new(iter),
            record: start,
            compare: Box::new(compare),
        }
    }
}

impl<T: Clone, G: PartialOrd> Iterator for Records<T, G> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let val = self.iter.next()?;
            let test = (self.compare)(&val);
            if test > self.record {
                self.record = test;
                return Some(val);
            }
        }
    }
}

pub struct HighWatermark<T> {
    iter: Box<dyn Iterator<Item = T>>,
    record: T,
}

impl<T> HighWatermark<T> {
    pub fn new<I>(iter: I, start: T) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        Self {
            iter: Box::new(iter),
            record: start,
        }
    }
}

impl<T: Clone + PartialOrd> Iterator for HighWatermark<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let val = self.iter.next()?;
            if val > self.record {
                self.record = val.clone();
                return Some(val);
            }
        }
    }
}
