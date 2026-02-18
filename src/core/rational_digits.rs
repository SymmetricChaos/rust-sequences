use num::{
    BigInt, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, PrimInt, Zero, rational::Ratio,
};
use std::fmt::Display;

pub fn integer_digits<T: Integer>(n: T, base: T) -> Vec<T> {
    let mut out = Vec::new();
    let mut n = n;
    while n > T::zero() {
        let (div, rem) = n.div_rem(&base);
        out.push(rem);
        n = div;
    }
    out
}

/// Digits of a non-negative fraction in a chosen base. If the fraction is less than one a leading zero is included to represent the integer part. If the fraction terminates infinite trailing zeroes are produced.
pub struct RationalDigits<T> {
    remdr: T,
    denom: T,
    base: T,
    numer: Vec<T>,
    leading_zero: bool,
}

impl<T: PrimInt + Integer + Display> RationalDigits<T> {
    pub fn new(numer: T, denom: T, base: T) -> Self {
        Self::from_ratio(Ratio::new(numer, denom), base)
    }

    pub fn from_ratio(q: Ratio<T>, base: T) -> Self {
        assert!(q >= Ratio::<T>::zero());
        let (i_part, f_part) = q.into_raw();
        let numer = integer_digits(i_part, base);
        let remdr = T::zero();
        Self {
            remdr,
            denom: f_part,
            base,
            numer,
            leading_zero: true,
        }
    }
}

impl RationalDigits<BigInt> {
    pub fn new_big<F: Integer + Clone, G>(numer: F, denom: F, base: G) -> Self
    where
        Ratio<BigInt>: From<Ratio<F>>,
        BigInt: From<F>,
        Ratio<BigInt>: From<Ratio<G>>,
        BigInt: From<G>,
    {
        Self::from_ratio_big(Ratio::new(numer, denom), base)
    }

    pub fn from_ratio_big<F: Clone + Integer, G>(q: Ratio<F>, base: G) -> Self
    where
        Ratio<BigInt>: From<Ratio<F>>,
        BigInt: From<F>,
        BigInt: From<G>,
    {
        let q: Ratio<BigInt> = q.into();
        assert!(q >= Ratio::<BigInt>::zero());
        let base = BigInt::from(base);
        let f = q.into_raw();
        let numer = integer_digits(f.0, base.clone());
        Self {
            remdr: BigInt::zero(),
            denom: f.1,
            base,
            numer,
            leading_zero: true,
        }
    }
}

impl<T: CheckedDiv + CheckedSub + CheckedMul + CheckedAdd + Zero> Iterator for RationalDigits<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.leading_zero {
            // Ugly loop to remove leading zeroes from the integer part of a value
            loop {
                self.remdr = self
                    .remdr
                    .checked_mul(&self.base)?
                    .checked_add(&self.numer.pop().unwrap_or(T::zero()))?;
                let out = self.remdr.checked_div(&self.denom)?;
                self.remdr = self.remdr.checked_sub(&self.denom.checked_mul(&out)?)?;
                if self.numer.is_empty() || !out.is_zero() {
                    self.leading_zero = false;
                    return Some(out);
                }
            }
        } else {
            self.remdr = self
                .remdr
                .checked_mul(&self.base)? // shift the digits of the remainder
                .checked_add(&self.numer.pop().unwrap_or(T::zero()))?; // pull down the next digit or zero if digits are used up
            let out = self.remdr.checked_div(&self.denom)?; // determine the next digit
            self.remdr = self.remdr.checked_sub(&self.denom.checked_mul(&out)?)?; // subtract from the remainder
            Some(out)
        }
    }
}

/// Convert a rational number to a string representing it in decimal with excactly the specificer number of digits after the decimal.
pub fn rational_decimal_string<
    T: Integer + CheckedDiv + CheckedSub + CheckedMul + CheckedAdd + Zero + Display + Clone,
>(
    ratio: Ratio<T>,
    digits: usize,
) -> Option<String> {
    let ten = T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one()
        + T::one();

    let (i_part, f_part) = ratio.into_raw();
    let mut numer = integer_digits(i_part, ten.clone());
    let denom = f_part;
    let mut remdr = T::zero();

    let mut dec_point_found = false;

    let mut output_string = String::new();

    // Remove leading zeroes or find the end of the integer part
    loop {
        remdr = remdr
            .checked_mul(&ten)?
            .checked_add(&numer.pop().unwrap_or(T::zero()))?;
        let digit = remdr.checked_div(&denom)?;
        remdr = remdr.checked_sub(&denom.checked_mul(&digit)?)?;
        if numer.is_empty() {
            output_string.push_str(&digit.to_string());
            output_string.push('.');
            dec_point_found = true;
            break;
        }
        if !digit.is_zero() {
            output_string.push_str(&digit.to_string());
            break;
        }
    }

    let mut ctr = 0;
    loop {
        let next = match numer.pop() {
            Some(n) => n,
            None => {
                if dec_point_found {
                    T::zero()
                } else {
                    dec_point_found = true;
                    output_string.push('.');
                    T::zero()
                }
            }
        };
        remdr = remdr
            .checked_mul(&ten)? // shift the digits of the remainder
            .checked_add(&next)?; // pull down the next digit or zero if digits are used up
        let digit = remdr.checked_div(&denom)?; // determine the next digit
        remdr = remdr.checked_sub(&denom.checked_mul(&digit)?)?; // subtract from the remainder
        output_string.push_str(&digit.to_string());
        if dec_point_found {
            ctr += 1;
        }
        if ctr >= digits {
            break;
        }
    }

    Some(output_string)
}

crate::check_sequences!(
    RationalDigits::new(665857, 941664, 10), 0, 20, [0,7,0,7,1,0,6,7,8,1,1,8,7,3,4,4,9,5,5,3];
    RationalDigits::new(10, 7, 10), 0, 10, [1, 4, 2, 8, 5, 7, 1, 4, 2, 8];
    RationalDigits::new(46, 3, 10), 0, 10, [1, 5, 3, 3, 3, 3, 3, 3, 3, 3];
    RationalDigits::new(1, 127, 10), 0, 20, [0, 0, 0, 7, 8, 7, 4, 0, 1, 5, 7, 4, 8, 0, 3, 1, 4, 9, 6, 0]; // check for correct leading zeroes, this is 0.007874...
);

crate::print_values!(
    digits, formatter "{}", sep " ";
    RationalDigits::new(665857, 941664, 2), 0, 20;
    RationalDigits::new(1, 4, 2), 0, 20;
    RationalDigits::new(10, 7, 10), 0, 20;
    RationalDigits::new(301, 3, 10), 0, 20;
    RationalDigits::new(1, 127, 10), 0, 20;
    RationalDigits::new(46, 3, 10), 0, 10;
);

#[cfg(test)]
#[test]
fn check_decimals() {
    println!(
        "{}\n{}\n{}",
        rational_decimal_string(Ratio::new(1, 127), 10).unwrap(),
        rational_decimal_string(Ratio::new(46, 3), 10).unwrap(),
        rational_decimal_string(Ratio::new(10, 7), 10).unwrap(),
    );
}
