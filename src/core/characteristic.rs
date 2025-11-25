pub struct CharacteristicSequence<T> {
    iter: Box<dyn Iterator<Item = T>>,
    phi: Box<dyn Fn(T) -> bool>,
}

impl<T> CharacteristicSequence<T> {
    pub fn new(iter: Box<dyn Iterator<Item = T>>, phi: Box<dyn Fn(T) -> bool>) -> Self {
        Self { iter, phi }
    }
}

impl<T> Iterator for CharacteristicSequence<T> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.iter.next()?;
        Some((self.phi)(n))
    }
}
