use num::{CheckedAdd, Integer};

pub struct RunLengthEncoding<T> {
    iter: Box<dyn Iterator<Item = T>>,
    val: Option<T>,
}

impl<T: CheckedAdd + Clone + Integer> RunLengthEncoding<T> {
    pub fn new<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = T> + 'static,
    {
        let val = iter.next();
        Self {
            val: val,
            iter: Box::new(iter),
        }
    }
}

impl<T: CheckedAdd + Clone + Integer> Iterator for RunLengthEncoding<T> {
    type Item = (T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.val.clone();
        let mut ctr = T::one();
        loop {
            let n = self.iter.next();
            if n == val {
                ctr = ctr.checked_add(&T::one())?;
            } else {
                self.val = n;
                break;
            }
        }

        Some((val.unwrap(), ctr))
    }
}

crate::print_sequences!(
    RunLengthEncoding::new([1, 1, 3, 3, 3, 3, 5, 5, 5, 5, 5, 5].into_iter()), 3, "{:?}", ", ";
);
