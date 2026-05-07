use num_format::ToFormattedString;
use rust_sequences::utils::divisibility::{is_prime, prime_factorization};
use std::{io::Write, time::Duration, u64};

fn _prime_factorization_timings() {
    let start_time = std::time::Instant::now();
    let mut longest = (std::time::Duration::ZERO, 0, vec![]);
    let mut total_time = std::time::Duration::ZERO;

    let heartbeat = Duration::from_secs(30);

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

        if i == u32::MAX as u64 {
            file.write_all(
                format!(
                    "reached u32::MAX after {:.0?}\n\n",
                    std::time::Instant::now() - start_time,
                )
                .as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }

        // Heartbeat
        if total_time > heartbeat {
            total_time -= heartbeat;
            file.write_all(
                format!(
                    "reached {} after {:.0?}\n\n",
                    i.to_formatted_string(&num_format::Locale::en),
                    std::time::Instant::now() - start_time,
                )
                .as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }
    }
}

pub fn _primality_check_time() {
    let start_time = std::time::Instant::now();
    let mut longest = (std::time::Duration::ZERO, 0);
    let mut total_time = std::time::Duration::ZERO;

    let heartbeat = Duration::from_secs(30);

    let path_and_name = format!("src/_primality_checking_u64s.txt");
    std::fs::File::create(&path_and_name).unwrap();
    let mut file = std::fs::File::options()
        .append(true)
        .open(&path_and_name)
        .unwrap();

    let start = u32::MAX as u64;
    let end = u64::MAX;

    for i in start..=end {
        // Timed section
        let t = std::time::Instant::now();
        let _fs = is_prime(i);
        let d = std::time::Instant::now() - t;

        total_time = total_time + d;

        // Record and print a new record for time to factor
        if d > longest.0 {
            longest = (d, i);
            // Save information to file
            file.write_all(
                format!(
                    "RECORD! {:<11?}    {} is prime\n\n",
                    longest.0,
                    longest.1.to_formatted_string(&num_format::Locale::en),
                )
                .as_bytes(),
            )
            .unwrap();
            file.flush().unwrap();
        }

        // Heartbeat
        if total_time > heartbeat {
            total_time -= heartbeat;
            file.write_all(
                format!(
                    "reached {} after {:.0?}\n\n",
                    i.to_formatted_string(&num_format::Locale::en),
                    std::time::Instant::now() - start_time,
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
    // _primality_check_time();
}
