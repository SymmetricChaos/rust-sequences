use num::integer::mod_floor;
use num::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Integer, One, Zero};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

//TODO: This could be made generic over architecture pointer size

/// ModInt uses an i32 internally so N should not be more than 46340 to avoid issues with multiplication.
/// If N is not prime, division will fail for some inputs.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModInt<const N: i32>(i32);

impl<const N: i32> ModInt<N> {
    /// Create a new ModInt from an i32 and force it to a valid value
    pub fn new(n: i32) -> Self {
        Self(mod_floor(n, N))
    }

    /// Create a new ModInt without checking the input
    pub fn new_raw(n: i32) -> Self {
        Self(n)
    }

    /// The multiplicative inverse if it exists
    pub fn recip(&self) -> Option<Self> {
        let egcd = self.0.extended_gcd(&N);
        if !egcd.gcd.is_one() {
            None
        } else {
            Some(Self::new(egcd.x))
        }
    }
}

impl<const N: i32> Zero for ModInt<N> {
    fn zero() -> Self {
        ModInt(0)
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl<const N: i32> One for ModInt<N> {
    fn one() -> Self {
        ModInt(1)
    }

    fn is_one(&self) -> bool {
        self.0 == 1
    }
}

impl<const N: i32> Display for ModInt<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<const N: i32> Debug for ModInt<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (mod {})", self.0, N)
    }
}

impl<const N: i32> Add for ModInt<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % N)
    }
}

impl<const N: i32> AddAssign for ModInt<N> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self((self.0 + rhs.0) % N)
    }
}

impl<const N: i32> CheckedAdd for ModInt<N> {
    fn checked_add(&self, v: &Self) -> Option<Self> {
        Some(Self(self.0.checked_add(v.0)? % N))
    }
}

impl<const N: i32> Sub for ModInt<N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl<const N: i32> SubAssign for ModInt<N> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self + -rhs;
    }
}

impl<const N: i32> CheckedSub for ModInt<N> {
    fn checked_sub(&self, v: &Self) -> Option<Self> {
        Some(Self((self.0 + N).checked_sub(v.0)? % N))
    }
}

impl<const N: i32> Mul for ModInt<N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self((self.0 * rhs.0) % N)
    }
}

impl<const N: i32> MulAssign for ModInt<N> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self((self.0 * rhs.0) % N)
    }
}

impl<const N: i32> CheckedMul for ModInt<N> {
    fn checked_mul(&self, v: &Self) -> Option<Self> {
        Some(Self(self.0.checked_mul(v.0)? % N))
    }
}

impl<const N: i32> Div for ModInt<N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.recip().unwrap()
    }
}

impl<const N: i32> DivAssign for ModInt<N> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self * rhs.recip().unwrap()
    }
}

impl<const N: i32> CheckedDiv for ModInt<N> {
    fn checked_div(&self, v: &Self) -> Option<Self> {
        self.checked_mul(&v.recip()?)
    }
}

impl<const N: i32> Neg for ModInt<N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.0)
    }
}

// Unclear if separating hashes by the value of N is meaningful
// impl<const N: i32> Hash for FiniteInt<N> {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.0.hash(state);
//         N.hash(state);
//     }
// }

#[cfg(test)]
mod math_tests {

    use super::*;

    #[test]
    fn mul() {
        type MI26 = ModInt<26>;
        let a = MI26::new(5);
        let b = MI26::new(7);
        println!("{} * {} = {}", a, b, a * b);
        let a = MI26::new(5);
        let b = MI26::new(21);
        println!("{} * {} = {}", a, b, a * b);
    }

    #[test]
    fn add() {
        type MI26 = ModInt<26>;
        let a = MI26::new(5);
        let b = MI26::new(7);
        println!("{} + {} = {}", a, b, a + b);
        let a = MI26::new(20);
        let b = MI26::new(10);
        println!("{} + {} = {}", a, b, a + b);
    }

    #[test]
    fn sub() {
        let a = ModInt::<26>(5);
        let b = ModInt::<26>(7);
        println!("{} - {} = {}", a, b, a - b);
    }

    #[test]
    fn div() {
        let a = ModInt::<26>(5);
        let b = ModInt::<26>(7);
        println!("{} / {} = {}", a, b, a / b);
    }

    #[test]
    fn recip() {
        let a = ModInt::<26>(5);
        println!("1 / {} = {:?}", a, a.recip());
        let b = ModInt::<26>(2);
        println!("1 / {} = {:?}", b, b.recip());
    }

    #[test]
    fn neg() {
        let a = ModInt::<26>(11);
        println!("{} + {} = {}", a, -a, a + -a);
    }
}
