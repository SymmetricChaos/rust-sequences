use num::{BigInt, Integer, Zero};

// Natural numbers that are not divisible twice by any natural number except one.
pub struct Squarefree {
    ctr: BigInt,
    squares: Vec<BigInt>,
    primes: crate::core::Prime<BigInt>,
}

impl Squarefree {
    pub fn new_big() -> Self {
        let mut primes = crate::core::Prime::new_big();
        primes.next();
        Self {
            ctr: BigInt::zero(),
            squares: vec![BigInt::from(4)],
            primes,
        }
    }
}

impl Iterator for Squarefree {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            self.ctr += 1;
            if &self.ctr >= self.squares.last().unwrap() {
                let n = self.primes.next().unwrap();
                self.squares.push(&n * &n);
            }
            for square in self.squares.iter() {
                if self.ctr.is_multiple_of(square) {
                    continue 'outer;
                }
            }
            break;
        }
        Some(self.ctr.clone())
    }
}

// Natural numbers that are divisible twice by at least one natural number other than one.
pub struct Squareful {
    ctr: BigInt,
    squares: Vec<BigInt>,
    primes: crate::core::Prime<BigInt>,
}

impl Squareful {
    pub fn new_big() -> Self {
        let mut primes = crate::core::Prime::new_big();
        primes.next();
        Self {
            ctr: BigInt::zero(),
            squares: vec![BigInt::from(4)],
            primes,
        }
    }
}

impl Iterator for Squareful {
    type Item = BigInt;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            self.ctr += 1;
            if &self.ctr >= self.squares.last().unwrap() {
                let n = self.primes.next().unwrap();
                self.squares.push(&n * &n);
            }
            for square in self.squares.iter() {
                if self.ctr.is_multiple_of(square) {
                    break 'outer;
                }
            }
        }
        Some(self.ctr.clone())
    }
}

crate::check_sequences!(
    Squarefree::new_big(), 0, 20, [1, 2, 3, 5, 6, 7, 10, 11, 13, 14, 15, 17, 19, 21, 22, 23, 26, 29, 30, 31];
    Squareful::new_big(), 0, 20, [4, 8, 9, 12, 16, 18, 20, 24, 25, 27, 28, 32, 36, 40, 44, 45, 48, 49, 50, 52];
);
