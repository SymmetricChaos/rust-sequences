use num::{
    BigInt, CheckedAdd, CheckedMul, FromPrimitive, Integer, One, PrimInt, Signed, Zero,
    rational::Ratio,
};

pub struct BernoulliPlus<T> {
    bernoullis: Vec<Ratio<T>>,
    m: usize,
}

impl<T: PrimInt + Signed + Integer> BernoulliPlus<T> {
    pub fn new() -> Self {
        Self {
            bernoullis: vec![Ratio::one()],
            m: 1,
        }
    }
}

impl BernoulliPlus<BigInt> {
    pub fn new_big() -> Self {
        Self {
            bernoullis: vec![Ratio::one()],
            m: 1,
        }
    }
}

impl<T: Clone + Integer + FromPrimitive + Zero + CheckedAdd + CheckedMul> Iterator
    for BernoulliPlus<T>
{
    type Item = Ratio<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.bernoullis.last().unwrap().clone();
        let mut sum = Ratio::<T>::zero();
        for k in 0..self.m {
            let b = num::integer::binomial(self.m, k);
            let frac = Ratio::new(T::one(), T::from_usize(self.m - k + 1)?)
                * self.bernoullis[k].clone()
                * T::from_usize(b)?;
            sum = sum.checked_add(&frac)?;
        }
        self.bernoullis.push(sum);
        Some(out)
    }
}

crate::print_values!(BernoulliPlus::<i32>::new(), 0, 20;);
