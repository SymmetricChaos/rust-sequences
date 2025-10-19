use num_format::ToFormattedString;
use rust_sequences::core::primality_utils::{
    is_prime, partial_trial_division, prime_factorization,
};
use std::{collections::BTreeMap, io::Write, u64};

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

fn prime_factorization_timings() {
    let start_time = std::time::Instant::now();
    let mut longest = (std::time::Duration::ZERO, 0, vec![]);
    let mut total_time = std::time::Duration::ZERO;

    let start = 1;
    let end = u64::MAX;

    let path_and_name = format!("src/_factorization_timings_1..txt");
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

        total_time = total_time + d;

        // Correctness check
        // Also prevents factorization from being optimized away
        let prod = fs.iter().fold(1, |acc, (pr, ct)| acc * pr.pow(*ct as u32));
        assert_eq!(i, prod);

        // Record and print a new record for time to factor
        if d > longest.0 {
            longest = (d, i, fs);
            // Save information to file
            file.write_all(
                format!(
                    "RECORD! {:<11?}    {} = {:?}\n\n",
                    longest.0,
                    longest.1.to_formatted_string(&num_format::Locale::en),
                    longest.2
                )
                .as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }

        // Heatbeat and average
        // Before 4,294,967,295 update every fifty million, after check only every 4,294,967,295
        if (i % 50_000_000 == 0 && i < u32::MAX as u64) || i % u32::MAX as u64 == 0 {
            file.write_all(
                format!(
                    "{}\nAVERAGE TIME TO FACTOR:  {:.4?}\nTOTAL TIME FACTORING:    {:.1?}\nTOTAL RUNNING TIME:      {:.1?}\n\n",
                    i.to_formatted_string(&num_format::Locale::en),
                    total_time.div_f64(i as f64),
                    total_time,
                    std::time::Instant::now() - start_time,
                )
                .as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }
    }
}

// run with cargo run --release
fn main() {
    // partial_factorization_density_test();
    prime_factorization_timings();
}
