use num::Zero;

macro_rules! add {
    ($s:ident, $r:ident) => {
        if $s.len() >= $r.len() {
            let mut out = $s.coef.clone();
            for (i, c) in $r.coef.iter().enumerate() {
                out[i] += c;
            }
            crate::utils::polynomial::Polynomial::new(out)
        } else {
            let mut out = $r.coef.clone();
            for (i, c) in $s.coef.iter().enumerate() {
                out[i] += c;
            }
            crate::utils::polynomial::Polynomial::new(out)
        }
    };
}

macro_rules! sub {
    ($s:ident, $r:ident) => {
        if $s.len() >= $r.len() {
            let mut out = $s.coef.clone();
            for (i, c) in $r.coef.iter().enumerate() {
                out[i] -= c;
            }
            crate::utils::polynomial::Polynomial::new(out)
        } else {
            let mut out = $r.coef.clone();
            for (i, c) in $s.coef.iter().enumerate() {
                out[i] -= c;
            }
            crate::utils::polynomial::Polynomial::new(out)
        }
    };
}

macro_rules! poly_arith {
    ($t:ty) => {
        impl num::Zero for crate::utils::polynomial::Polynomial<$t> {
            fn zero() -> Self {
                crate::utils::polynomial::Polynomial::new_raw(vec![])
            }

            fn is_zero(&self) -> bool {
                self.len() == 0
            }
        }

        impl num::One for crate::utils::polynomial::Polynomial<$t> {
            fn one() -> Self {
                crate::utils::polynomial::Polynomial::new_raw(vec![<$t>::one()])
            }

            fn is_one(&self) -> bool {
                self.len() == 1 || self.coef[0].is_one()
            }
        }

        impl std::ops::Add for crate::utils::polynomial::Polynomial<$t> {
            type Output = crate::utils::polynomial::Polynomial<$t>;

            fn add(self, rhs: Self) -> Self::Output {
                add!(self, rhs)
            }
        }

        impl std::ops::Add<&crate::utils::polynomial::Polynomial<$t>>
            for &crate::utils::polynomial::Polynomial<$t>
        {
            type Output = crate::utils::polynomial::Polynomial<$t>;

            fn add(self, rhs: &crate::utils::polynomial::Polynomial<$t>) -> Self::Output {
                add!(self, rhs)
            }
        }

        impl std::ops::Add<&crate::utils::polynomial::Polynomial<$t>>
            for crate::utils::polynomial::Polynomial<$t>
        {
            type Output = crate::utils::polynomial::Polynomial<$t>;

            fn add(self, rhs: &crate::utils::polynomial::Polynomial<$t>) -> Self::Output {
                add!(self, rhs)
            }
        }

        impl std::ops::Sub for crate::utils::polynomial::Polynomial<$t> {
            type Output = crate::utils::polynomial::Polynomial<$t>;

            fn sub(self, rhs: Self) -> Self::Output {
                sub!(self, rhs)
            }
        }

        impl std::ops::Sub<&crate::utils::polynomial::Polynomial<$t>>
            for &crate::utils::polynomial::Polynomial<$t>
        {
            type Output = crate::utils::polynomial::Polynomial<$t>;

            fn sub(self, rhs: &crate::utils::polynomial::Polynomial<$t>) -> Self::Output {
                sub!(self, rhs)
            }
        }

        impl std::ops::Sub<&crate::utils::polynomial::Polynomial<$t>>
            for crate::utils::polynomial::Polynomial<$t>
        {
            type Output = crate::utils::polynomial::Polynomial<$t>;

            fn sub(self, rhs: &crate::utils::polynomial::Polynomial<$t>) -> Self::Output {
                sub!(self, rhs)
            }
        }

        impl std::ops::Mul for crate::utils::polynomial::Polynomial<$t> {
            type Output = crate::utils::polynomial::Polynomial<$t>;

            fn mul(self, rhs: Self) -> Self::Output {
                let mut out_coefs = vec![<$t>::zero(); self.len() + rhs.len()];

                for (idx_a, a) in self.iter().enumerate() {
                    for (idx_b, b) in rhs.iter().enumerate() {
                        out_coefs[idx_a + idx_b] += a * b;
                    }
                }

                crate::utils::polynomial::Polynomial::new(out_coefs)
            }
        }

        impl std::ops::Mul<&crate::utils::polynomial::Polynomial<$t>>
            for &crate::utils::polynomial::Polynomial<$t>
        {
            type Output = crate::utils::polynomial::Polynomial<$t>;

            fn mul(self, rhs: &crate::utils::polynomial::Polynomial<$t>) -> Self::Output {
                let mut out_coefs = vec![<$t>::zero(); self.len() + rhs.len()];

                for (idx_a, a) in self.iter().enumerate() {
                    for (idx_b, b) in rhs.iter().enumerate() {
                        out_coefs[idx_a + idx_b] += a * b;
                    }
                }

                crate::utils::polynomial::Polynomial::new(out_coefs)
            }
        }

        impl std::ops::Mul<&crate::utils::polynomial::Polynomial<$t>>
            for crate::utils::polynomial::Polynomial<$t>
        {
            type Output = crate::utils::polynomial::Polynomial<$t>;

            fn mul(self, rhs: &crate::utils::polynomial::Polynomial<$t>) -> Self::Output {
                let mut out_coefs = vec![<$t>::zero(); self.len() + rhs.len()];

                for (idx_a, a) in self.iter().enumerate() {
                    for (idx_b, b) in rhs.iter().enumerate() {
                        out_coefs[idx_a + idx_b] += a * b;
                    }
                }

                crate::utils::polynomial::Polynomial::new(out_coefs)
            }
        }

        impl std::ops::Neg for crate::utils::polynomial::Polynomial<$t> {
            type Output = Self;

            fn neg(mut self) -> Self::Output {
                for i in self.coef.iter_mut() {
                    *i = -(i.clone());
                }
                self
            }
        }

        impl crate::utils::polynomial::Polynomial<$t> {
            /// Evaluation of the polynomial at x by Horner's method.
            pub fn eval(&self, x: &$t) -> $t {
                let mut total = <$t>::zero();
                for c in self.coef.iter().rev() {
                    total = total * x + c;
                }
                total
            }
        }
    };
}

poly_arith!(i32);
poly_arith!(i64);
poly_arith!(num::BigInt);
poly_arith!(num::rational::Ratio<i32>);
poly_arith!(num::rational::Ratio<i64>);
poly_arith!(num::BigRational);
