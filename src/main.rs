use std::{collections::BTreeMap, io::Write, u64};

use rust_sequences::core::{
    is_prime, partial_trial_division, primality_utils::prime_factorization,
};

fn partial_factorization_density_test() {
    let start = 1;
    let end = u32::MAX as u64;
    let mut num_factored = 0;
    std::fs::File::create(format!("src/_partial_factorization_density.txt")).unwrap();
    for i in start..=end {
        let mut map = BTreeMap::new();
        let r = partial_trial_division(i, &mut map);
        if is_prime(r) || r == 1 {
            num_factored += 1;
        }

        // Save information to file
        if i % 10_000_000 == 0 || i == end {
            let mut file = std::fs::File::options()
                .append(true)
                .open("src/_partial_factorization_density.txt")
                .unwrap();
            file.write_all(format!("searched range {start}..={i}\n").as_bytes())
                .unwrap();
            file.write_all(
                format!(
                    "density of integers fully factored: {}\n\n",
                    num_factored as f64 / i as f64,
                )
                .as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }
    }
}

fn prime_factorization_speed_test() {
    let mut total_time = 0;
    let mut longest_time = (0, 0, vec![]);
    let start = 1;
    let end = u32::MAX as u64;
    let mut ctr = 0;
    std::fs::File::create("src/_prime_factorization_speed_test.txt").unwrap();
    for i in start..=end {
        ctr += 1;

        // Timed section
        let t = std::time::Instant::now();
        let fs = prime_factorization(i);
        let d = std::time::Instant::now() - t;

        // Correctness checks
        // Also prevents factorization from being optimized away
        assert!(
            (is_prime(i) && fs.len() == 1 && fs.iter().next().unwrap().1 == 1)
                || (!is_prime(i) && (fs.len() != 1 || fs.iter().next().unwrap().1 > 1)),
            "factorization of {} is wrong found: {:?}",
            i,
            fs
        );
        let prod = fs.iter().fold(1, |acc, (pr, ct)| acc * pr.pow(*ct as u32));
        assert_eq!(i, prod);

        // Convert time to microseconds and check if new record had been found
        let m = d.as_micros();
        if m > longest_time.0 {
            longest_time = (m, i, fs)
        }
        total_time = total_time + &m;

        // Save information to file
        if i % 1_000_000 == 0 || i == end {
            let mut file = std::fs::File::options()
                .append(true)
                .open("src/_prime_factorization_speed_test.txt")
                .unwrap();
            file.write_all(format!("searched range {start}..={i}\n").as_bytes())
                .unwrap();
            file.write_all(
                format!(
                    "longest time to factor: {:?}μs ({} = {:?})\n",
                    longest_time.0, longest_time.1, longest_time.2
                )
                .as_bytes(),
            )
            .unwrap();
            file.write_all(
                format!("average time to factor: {:?}μs\n\n", total_time / ctr).as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }
    }
}

// run with cargo run --release
fn main() {
    // prime_factorization_speed_test()
    partial_factorization_density_test();
}
