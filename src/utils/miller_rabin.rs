use num::integer::gcd;

use crate::utils::exp_by_squaring::pow_mod;

/// These primes are sufficient witnessses to do a deterministic Miller-Rabin test for all u64.
pub(super) const MR_WITNESSES_U64: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub(super) enum MRTest {
    Prime,
    Composite(Option<u64>),
}

/// A Miller-Rabin test that assumes the input is an odd number greater than 2.
pub(super) fn miller_rabin(n: u64) -> MRTest {
    let mut d = (n - 1) / 2;
    let r = 1_u64 + d.trailing_zeros() as u64;
    d >>= d.trailing_zeros();

    'outer: for w in MR_WITNESSES_U64.into_iter() {
        let mut x = pow_mod(w, d, n);

        if x == 1 || x == n - 1 {
            continue 'outer;
        }

        for _ in 0..r - 1 {
            let t = pow_mod(x, 2, n);

            if t == n - 1 {
                continue 'outer;
            }

            if t == 1 && x != 1 && x != (n - 1) {
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
    // for i in 100..100_000 {
    //     match miller_rabin(i) {
    //         MRTest::Prime => continue,
    //         MRTest::Composite(c) => match c {
    //             Some(f) => {
    //                 println!("{} = {} * {}", i, f, i / f);
    //                 assert_eq!(i, (i / f) * f);
    //             }
    //             None => continue,
    //         },
    //     }
    // }
    println!("{:?}", miller_rabin(41));
    println!("{:?}", miller_rabin(5461));
}
