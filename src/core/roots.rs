use num::{
    BigInt, CheckedAdd, CheckedDiv, CheckedMul, FromPrimitive, Integer, PrimInt, rational::Ratio,
};

/// Convergents of the principal square root of a rational number by Newton's Method.
pub struct SquareRoot<T> {
    convergent: Ratio<T>,
    s: Ratio<T>,
}

impl<T: PrimInt + Integer> SquareRoot<T> {
    pub fn new_prim(numer: T, denom: T) -> Self {
        let n = numer;
        let d = denom;
        Self {
            convergent: Ratio::new(n.clone(), d.clone()),
            s: Ratio::new(n.clone(), d.clone()),
        }
    }
}

impl SquareRoot<BigInt> {
    pub fn new<N>(numer: N, denom: N) -> Self
    where
        BigInt: From<N>,
    {
        let n = BigInt::from(numer);
        let d = BigInt::from(denom);
        Self {
            convergent: Ratio::new(n.clone(), d.clone()),
            s: Ratio::new(n.clone(), d.clone()),
        }
    }
}

impl<T: Clone + CheckedDiv + CheckedAdd + CheckedMul + Integer + FromPrimitive> Iterator
    for SquareRoot<T>
{
    type Item = Ratio<T>;

    // (x+s/x)/2
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.convergent.clone();
        let half = Ratio::new(T::from_i32(1)?, T::from_i32(2)?);
        self.convergent = self
            .convergent
            .checked_add(&self.s.checked_div(&self.convergent)?)?
            .checked_mul(&half)?;
        Some(out)
    }
}

/// Convergents of the principal cube root of a rational number by Newton's Method.
pub struct CubeRoot<T> {
    convergent: Ratio<T>,
    s: Ratio<T>,
}

impl<T: PrimInt + Integer> CubeRoot<T> {
    pub fn new_prim(num: T, den: T) -> Self {
        let n = num;
        let d = den;
        Self {
            convergent: Ratio::new(n.clone(), d.clone()),
            s: Ratio::new(n.clone(), d.clone()),
        }
    }
}

impl CubeRoot<BigInt> {
    pub fn new<N>(num: N, den: N) -> Self
    where
        BigInt: From<N>,
    {
        let n = BigInt::from(num);
        let d = BigInt::from(den);
        Self {
            convergent: Ratio::new(n.clone(), d.clone()),
            s: Ratio::new(n.clone(), d.clone()),
        }
    }
}

impl<T: Clone + CheckedDiv + CheckedAdd + CheckedMul + Integer + FromPrimitive> Iterator
    for CubeRoot<T>
{
    type Item = Ratio<T>;

    // (s/x^2 + 2x)/3
    fn next(&mut self) -> Option<Self::Item> {
        let out = self.convergent.clone();
        let two_x = self.convergent.checked_add(&self.convergent)?;
        let third = Ratio::new(T::from_i32(1)?, T::from_i32(3)?);
        let squ = self.convergent.checked_mul(&self.convergent)?;
        self.convergent = self
            .s
            .checked_div(&squ)?
            .checked_add(&two_x)?
            .checked_mul(&third)?;
        Some(out)
    }
}

// /// Convergents of the principal nth root of a rational number by Newton's Method.
// pub struct NthRoot<T> {
//     convergent: Ratio<T>,
//     s: Ratio<T>,
//     n_recip: Ratio<T>,
//     n_m1: Ratio<T>,
// }

// impl<T: PrimInt + Integer + FromPrimitive + Bounded> NthRoot<T> {
//     pub fn new_prim(num: T, den: T, nth: u32) -> Self {
//         assert!(nth > 2);
//         assert!(
//             T::from_u32(nth).unwrap() <= T::max_value(),
//             "nth must fit within the type chosen"
//         );
//         let n = num;
//         let d = den;
//         let n_recip = Ratio::new(T::one(), T::from_u32(nth).unwrap());
//         let n_m1 = Ratio::new(T::from_u32(nth - 1).unwrap(), T::one());
//         Self {
//             convergent: Ratio::new(n.clone(), d.clone()),
//             s: Ratio::new(n.clone(), d.clone()),
//             n_recip,
//             n_m1,
//         }
//     }
// }

// impl NthRoot<BigInt> {
//     pub fn new<N>(num: N, den: N, nth: u32) -> Self
//     where
//         BigInt: From<N>,
//     {
//         assert!(nth > 2);
//         let n = BigInt::from(num);
//         let d = BigInt::from(den);
//         let n_recip = Ratio::new(BigInt::one(), BigInt::from_u32(nth).unwrap());
//         let n_m1 = Ratio::new(BigInt::from_u32(nth - 1).unwrap(), BigInt::one());
//         Self {
//             convergent: Ratio::new(n.clone(), d.clone()),
//             s: Ratio::new(n.clone(), d.clone()),
//             n_recip,
//             n_m1,
//         }
//     }
// }

// impl<T: Clone + CheckedDiv + CheckedAdd + CheckedMul + Integer + FromPrimitive> Iterator
//     for NthRoot<T>
// {
//     type Item = Ratio<T>;

//     // (s/x^2 + 2x)/3
//     fn next(&mut self) -> Option<Self::Item> {
//         let out = self.convergent.clone();

//         self.convergent = self
//             .s
//             .checked_div(&squ)?
//             .checked_add(&two_x)?
//             .checked_mul(&self.n_recip)?;
//         Some(out)
//     }
// }
