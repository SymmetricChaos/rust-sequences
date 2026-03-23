use itertools::Itertools;
use num::{BigInt, CheckedAdd, CheckedSub, Integer, One};

/// The number of partitons for each integer.
/// 1, 1, 2, 3, 5, 7, 11, 15, 22, 30...
pub struct Partition<T> {
    values: Vec<T>,
    ctr: usize,
}

impl<T: One> Partition<T> {
    pub fn new() -> Self {
        Self {
            values: vec![T::one()],
            ctr: 0,
        }
    }
}

impl Partition<BigInt> {
    pub fn new_big() -> Self {
        Self::new()
    }
}

impl<T: Clone + Integer + CheckedAdd + CheckedSub> Iterator for Partition<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.values[self.ctr].clone();

        self.ctr += 1;
        let mut parts = T::zero();
        let mut sign = 0;

        for p in crate::figurate::GeneralizedPentagonal::<i64>::new().skip(1) {
            let p = TryInto::<usize>::try_into(p).ok()?;
            if let Some(idx) = self.ctr.checked_sub(p) {
                if sign < 2 {
                    parts = parts.checked_add(&self.values[idx])?;
                } else {
                    parts = parts.checked_sub(&self.values[idx])?;
                }
                sign = (sign + 1) % 4;
            } else {
                break;
            }
        }

        self.values.push(parts.clone());

        Some(out)
    }
}

// https://github.com/quadrupleslap/integer-partitions/blob/master/src/lib.rs
/// The partitions of a non-negative integer.
pub struct PartitionsN {
    k: usize,
    y: usize,
    parts: Vec<usize>,
    state: State,
}

enum State {
    A,
    B { x: usize, l: usize },
}

impl PartitionsN {
    pub fn new(n: usize) -> Self {
        Self {
            parts: vec![0; n + 1],
            k: if n == 0 { 0 } else { 1 },
            y: if n == 0 { 0 } else { n - 1 },
            state: State::A,
        }
    }
}

impl Iterator for PartitionsN {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let PartitionsN {
            ref mut parts,
            ref mut k,
            ref mut y,
            ref mut state,
        } = *self;
        match *state {
            State::A => {
                if *k == 0 {
                    if parts.len() == 1 {
                        parts.pop();
                        Some(vec![])
                    } else {
                        None
                    }
                } else {
                    *k -= 1;
                    let x = parts[*k] + 1;

                    while 2 * x <= *y {
                        parts[*k] = x;
                        *y -= x;
                        *k += 1;
                    }

                    let l = *k + 1;

                    if x <= *y {
                        parts[*k] = x;
                        parts[l] = *y;
                        *state = State::B { x, l };
                        Some(parts[..*k + 2].to_vec())
                    } else {
                        parts[*k] = x + *y;
                        *y = x + *y - 1;
                        Some(parts[..*k + 1].to_vec())
                    }
                }
            }
            State::B { mut x, l } => {
                x += 1;
                *y -= 1;

                if x <= *y {
                    parts[*k] = x;
                    parts[l] = *y;
                    *state = State::B { x, l };
                    Some(parts[..*k + 2].to_vec())
                } else {
                    parts[*k] = x + *y;
                    *y = x + *y - 1;
                    *state = State::A;
                    Some(parts[..*k + 1].to_vec())
                }
            }
        }
    }
}

/// The partitions of each non-negative integer.
pub struct Partitions {
    ctr: usize,
}

impl Partitions {
    pub fn new() -> Self {
        Self { ctr: 0 }
    }
}

impl Iterator for Partitions {
    type Item = Vec<Vec<usize>>;

    fn next(&mut self) -> Option<Self::Item> {
        let out = PartitionsN::new(self.ctr.clone()).collect_vec();
        self.ctr = self.ctr.checked_add(1)?;
        Some(out)
    }
}

crate::check_iteration_times!(
    Partition::new_big(), 405;
    Partition::<u64>::new(), 405;
);

crate::check_sequences!(
    Partition::new_big(), [1, 1, 2, 3, 5, 7, 11, 15, 22, 30];
    Partition::<u64>::new(), [1, 1, 2, 3, 5, 7, 11, 15, 22, 30];
);

crate::print_sequences!(
    print_arrays;
    PartitionsN::new(5), 10, "{:?}", "\n";
    Partitions::new(), 5, "{:?}", "\n";
);
