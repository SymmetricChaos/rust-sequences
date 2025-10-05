use std::{collections::BTreeMap, io::Write, u64};

use rust_sequences::core::{
    is_prime, partial_trial_division, primality_utils::prime_factorization,
};

fn partial_factorization_density_test() {
    let start = 1;
    let end = u32::MAX as u64;
    let mut num_factored = 0;

    let path_and_name = format!("src/_partial_factorization_density_{start}..={end}.txt");
    std::fs::File::create(&path_and_name).unwrap();
    let mut file = std::fs::File::options()
        .append(true)
        .open(&path_and_name)
        .unwrap();
    file.write_all(
        b"factorization using only partial trial division up to 37 and a primality test\n\n",
    )
    .unwrap();
    for i in start..=end {
        let mut map = BTreeMap::new();
        let r = partial_trial_division(i, &mut map);
        if is_prime(r) || r == 1 {
            num_factored += 1;
        }

        // Save information to file
        if i % 10_000_000 == 0
            || i == end
            || i == u8::MAX as u64
            || i == u16::MAX as u64
            || i == u32::MAX as u64
        {
            file.write_all(format!("searched {start}..={i}\n").as_bytes())
                .unwrap();
            file.write_all(
                format!("density: {:.5}\n\n", num_factored as f64 / i as f64,).as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }
    }
}

fn average_time_to_factor() {
    let mut total_time = std::time::Duration::ZERO;
    let mut longest_time = (0, 0, vec![]);
    let start = 1;
    let end = u32::MAX as u64;

    let path_and_name = format!("src/_average_time_to_factor_{start}..={end}.txt");
    std::fs::File::create(&path_and_name).unwrap();
    let mut file = std::fs::File::options()
        .append(true)
        .open(&path_and_name)
        .unwrap();
    for i in start..=end {
        // Timed section
        let t = std::time::Instant::now();
        let fs = prime_factorization(i);
        let d = std::time::Instant::now() - t;

        // Correctness check
        // Also prevents factorization from being optimized away
        let prod = fs.iter().fold(1, |acc, (pr, ct)| acc * pr.pow(*ct as u32));
        assert_eq!(i, prod);

        // Convert time to microseconds and check if new record had been found
        let m = d.as_micros();
        if m > longest_time.0 {
            longest_time = (m, i, fs)
        }
        total_time = total_time + d;

        // Save information to file
        if i % 1_000_000 == 0 || i == end {
            file.write_all(format!("searched {start}..={i}\n").as_bytes())
                .unwrap();
            file.write_all(format!("average: {:.5?}\n\n", total_time.div_f64(i as f64)).as_bytes())
                .unwrap();
            file.flush().unwrap();
        }
    }
}

fn hardest_to_factor() {
    let mut longest_time = (std::time::Duration::ZERO, 0, vec![]);
    let start = 1;
    let end = u64::MAX;

    let path_and_name = format!("src/_hardest_to_factor_range_{start}..={end}.txt");
    std::fs::File::create(&path_and_name).unwrap();
    let mut file = std::fs::File::options()
        .append(true)
        .open(&path_and_name)
        .unwrap();
    for i in start..=end {
        // Timed section
        let t = std::time::Instant::now();
        let fs = prime_factorization(i);
        let d = std::time::Instant::now() - t;

        // Correctness check
        // Also prevents factorization from being optimized away
        let prod = fs.iter().fold(1, |acc, (pr, ct)| acc * pr.pow(*ct as u32));
        assert_eq!(i, prod);

        // Convert time to microseconds and check if new record had been found
        if d > longest_time.0 {
            longest_time = (d, i, fs);
            // Save information to file
            file.write_all(
                format!(
                    "{:<15?}  {} = {:?}\n",
                    longest_time.0, longest_time.1, longest_time.2
                )
                .as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }

        // Heatbeat
        if i % 1_000_000 == 0 || i == end {
            file.write_all(b"...\n").unwrap();
            file.flush().unwrap();
        }
    }
}

fn prime_factorization_times() {
    let start = 2_u64.pow(60);
    let end = start + 2_u64.pow(16);

    let path_and_name = format!("src/_prime_factorization_times_range_{start}..={end}.txt");
    std::fs::File::create(&path_and_name).unwrap();
    let mut file = std::fs::File::options()
        .append(true)
        .open(&path_and_name)
        .unwrap();
    for i in start..=end {
        // Timed section
        let t = std::time::Instant::now();
        let fs = prime_factorization(i);
        let d = std::time::Instant::now() - t;

        // Correctness check
        // Also prevents factorization from being optimized away
        let prod = fs.iter().fold(1, |acc, (pr, ct)| acc * pr.pow(*ct as u32));
        assert_eq!(i, prod);

        // Save information to file
        file.write_all(format!("{:<15?}  {} = {:?}\n", d, i, fs).as_bytes())
            .unwrap();
        file.flush().unwrap();
    }
}

// run with cargo run --release
fn main() {
    average_time_to_factor();
    // partial_factorization_density_test();
    // hardest_to_factor();
    // prime_factorization_times();
}
