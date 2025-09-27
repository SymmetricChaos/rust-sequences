use std::io::Write;

use rust_sequences::core::{is_prime, primality_utils::prime_factorization};

fn prime_factorization_speed_test() {
    let mut sum = 0;
    let mut longest = (0, 0, vec![]);
    let start = 1;
    std::fs::File::create("src/_prime_factorization_speed_test.txt").unwrap();
    for i in start..=(u32::MAX as u64) {
        let t = std::time::Instant::now();
        let fs = prime_factorization(i);
        let d = std::time::Instant::now() - t;

        // Correctness check
        assert!(
            (is_prime(i) && fs.len() == 1 && fs.iter().next().unwrap().1 == 1)
                || (!is_prime(i) && (fs.len() != 1 || fs.iter().next().unwrap().1 > 1)),
            "factorization of {} is wrong found: {:?}",
            i,
            fs
        );
        let prod = fs.iter().fold(1, |acc, (pr, ct)| acc * pr.pow(*ct as u32));
        assert_eq!(i, prod);

        let m = d.as_micros();
        if m > longest.0 {
            longest = (m, i, fs)
        }
        sum = sum + &m;

        if i % 1_000_000 == 0 {
            let mut file = std::fs::File::options()
                .append(true)
                .open("src/_prime_factorization_speed_test.txt")
                .unwrap();
            file.write_all(format!("searched range {start}..={i}\n").as_bytes())
                .unwrap();
            file.write_all(
                format!(
                    "longest time to factor: {:?}us ({} = {:?})\n",
                    longest.0, longest.1, longest.2
                )
                .as_bytes(),
            )
            .unwrap();
            file.write_all(
                format!(
                    "average time to factor: {:?}us\n\n",
                    sum / (i - start) as u128
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
    prime_factorization_speed_test()
}
