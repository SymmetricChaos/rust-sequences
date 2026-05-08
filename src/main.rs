use num_format::ToFormattedString;
use rust_sequences::utils::divisibility::{is_prime, prime_factorization};
use std::{io::Write, time::Duration, u64};

fn _prime_factorization_timings() {
    let start_time = std::time::Instant::now();
    let mut record = (std::time::Duration::ZERO, 0, vec![]);
    let mut factoring_time = std::time::Duration::ZERO;

    let heartbeat = Duration::from_secs(60);

    let start = 1;
    let end = u64::MAX;

    let path_and_name = format!("src/_factorization_timings_1.txt");
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

        factoring_time = factoring_time + d;

        // Correctness checks
        // Also prevents factorization from being optimized away
        let prod = fs.iter().fold(1, |acc, (pr, ct)| acc * pr.pow(*ct as u32));
        assert!(
            i == prod,
            "\nCORRECTNESS CHECK FAILED\nproduct should be {i} but found {prod}\n"
        );
        for f in fs.iter() {
            assert!(
                is_prime(f.0),
                "\nCORRECTNESS CHECK FAILED\nfound non-prime in factorization of {i}\n"
            )
        }

        // Record and print a new record for time to factor
        if d > record.0 {
            record = (d, i, fs);
            // Save information to file
            file.write_all(
                format!(
                    "RECORD! {:<11?}    {} = {:?}\n\n",
                    record.0,
                    record.1.to_formatted_string(&num_format::Locale::en),
                    record.2
                )
                .as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }

        if i == u32::MAX as u64 {
            file.write_all(
                format!(
                    "reached u32::MAX after {:.0?}\n\n",
                    std::time::Instant::now().duration_since(start_time),
                )
                .as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }

        // Heartbeat
        if factoring_time > heartbeat {
            factoring_time -= heartbeat;
            file.write_all(
                format!(
                    "reached {} after {:.0?}\n\n",
                    i.to_formatted_string(&num_format::Locale::en),
                    std::time::Instant::now().duration_since(start_time),
                )
                .as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }
    }
}

// cargo run --release
fn main() {
    _prime_factorization_timings();
}
