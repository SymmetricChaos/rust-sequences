#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MRTest {
    Prime,
    Composite(Option<u64>),
}

pub fn miller_rabin_factor(n: u64) -> MRTest {
    let mut d = (n - 1) / 2;
    let r = 1_u64 + d.trailing_zeros() as u64;
    d >>= d.trailing_zeros();

    'outer: for w in WITNESSES_U64.into_iter() {
        let mut x = pow_mod(w, d, n);

        if x == 1 || x == n - 1 {
            continue 'outer;
        }

        for _ in 0..r - 1 {
            let t = pow_mod(x, 2, n);

            if x == n - 1 {
                continue 'outer;
            }

            if t == 1 && x != 1 && x != n - 1 {
                return MRTest::Composite(Some(gcd(x - 1, n)));
            }

            x = t;
        }
        return MRTest::Composite(None);
    }
    MRTest::Prime
}

#[cfg(test)]
#[test]
#[ignore = "visualization"]
fn factor_test() {
    for i in 10..1_000_000 {
        match miller_rabin_factor(i) {
            MRTest::Prime => continue,
            MRTest::Composite(c) => match c {
                Some(f) => println!("{i} is divisible by {f}"),
                None => continue,
            },
        }
    }
}
