/// A linear congruential generator. A kind of simple PRNG.
pub struct Lcg {
    n: u64,
    a: u64,
    c: u64,
    m: u64,
}

impl Lcg {
    pub fn new(n: u32, a: u32, c: u32, m: u32) -> Self {
        Self {
            n: n as u64,
            a: a as u64,
            c: c as u64,
            m: m as u64,
        }
    }
}

impl Iterator for Lcg {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.n;
        self.n = (self.n * self.a + self.c) % self.m;
        Some(out)
    }
}

crate::check_sequences!(
    // Lehmer's LCG
    Lcg::new(47594118 , 23, 0, 100000001), [47594118, 94664704, 77288171, 77627916, 85442051, 65167154, 98844528, 73424122, 88754790, 41360150, 51283441, 79519132, 28940018, 65620408, 9269369, 13195485, 3496152, 80411496, 49464390, 37680959, 66662049, 33227112, 64223569];
    // The infamous RANDU LCG
    Lcg::new(1, 65539, 0, 1<<31), [1, 65539, 393225, 1769499, 7077969, 26542323, 95552217, 334432395, 1146624417, 1722371299, 14608041, 1766175739, 1875647473, 1800754131, 366148473, 1022489195, 692115265, 1392739779, 2127401289, 229749723, 1559239569, 845238963, 1775695897, 899541067, 153401569];
);
