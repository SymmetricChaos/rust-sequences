pub struct Golomb {
    ctrs: Vec<u64>,
    idx: usize,
    ctr: u64,
}

impl Golomb {
    pub fn new() -> Self {
        Self {
            ctrs: vec![],
            idx: 0,
            ctr: 1,
        }
    }
}

impl Iterator for Golomb {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

crate::check_sequences!(
    Golomb::new(), [1, 2, 2, 3, 3, 4, 4, 4, 5, 5, 5, 6, 6, 6, 6, 7, 7, 7, 7, 8, 8, 8, 8, 9, 9, 9, 9, 9, 10, 10, 10, 10, 10];
);
