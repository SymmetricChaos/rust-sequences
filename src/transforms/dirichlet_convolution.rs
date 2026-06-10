use crate::core::traits::Increment;
use num::{CheckedAdd, CheckedMul, Zero};

/// The Dirichlet convolution.
pub struct DirichletConvolution<T> {
    ctr: usize,
    f_terms: Vec<T>,
    g_terms: Vec<T>,
    f: Box<dyn Iterator<Item = T>>,
    g: Box<dyn Iterator<Item = T>>,
}

impl<T> DirichletConvolution<T> {
    pub fn new<F, G>(f: F, g: G) -> Self
    where
        F: Iterator<Item = T> + 'static,
        G: Iterator<Item = T> + 'static,
    {
        Self {
            ctr: 1,
            f_terms: Vec::new(),
            g_terms: Vec::new(),
            f: Box::new(f),
            g: Box::new(g),
        }
    }
}

impl<T: Clone + Zero + CheckedAdd + CheckedMul> Iterator for DirichletConvolution<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut out = T::zero();

        self.f_terms.push(self.f.next()?);
        self.g_terms.push(self.g.next()?);

        let n = self.ctr;

        for d in 1..=n {
            if n.is_multiple_of(d) {
                out =
                    out.checked_add(&self.f_terms[d - 1].checked_mul(&self.g_terms[(n / d) - 1])?)?;
            }
        }
        self.ctr.incr()?;

        Some(out)
    }
}
