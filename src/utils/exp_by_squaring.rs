pub fn pow_mod(n: u64, p: u64, m: u64) -> u64 {
    if p == 0 {
        return 1;
    }
    if p == 1 {
        return n % m;
    }
    if p % 2 == 0 {
        return pow_mod((n * n) % m, p / 2, m) % m;
    } else {
        return (n * pow_mod((n * n) % m, (p - 1) / 2, m)) % m;
    }
}

#[test]
fn mul() {
    println!("{} = {}", pow_mod(2, 10, u64::MAX), 2_u64.pow(10));
    println!("{} = {}", pow_mod(2, 10, 123), 2_u64.pow(10) % 123);
    println!("{} = {}", pow_mod(2, 31, 123), 2_u64.pow(31) % 123);
    println!("{} = {}", pow_mod(2, 61, 5181), 2_u64.pow(61) % 5181);
    println!("{} ", pow_mod(2, 183, 5181));
}
