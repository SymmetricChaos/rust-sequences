pub mod bell;
pub mod catalan;
pub mod collatz;
pub mod core;
pub mod derangement;
pub mod digital_product;
pub mod digital_sum;
pub mod eulers_number;
pub mod evil;
pub mod farey;
pub mod fibonacci;
pub mod figurate;
pub mod gray;
pub mod harmonic;
pub mod jordan_polya;
pub mod lucas;
pub mod lucas_sequence;
pub mod mobius;
pub mod nonhypotenuse;
pub mod odious;
pub mod partition;
pub mod pell;
pub mod phythagorean;
pub mod pi;
pub mod playground;
pub mod repint;
pub mod rowland;
pub mod smooth;
pub mod squarefree;
pub mod strings;
pub mod thue_morse;
pub mod trig;
pub mod weyl;
pub mod zeta;

// #[macro_export]
// macro_rules! generic_two {
//     ($n:ty) => {
//         <$n>::one() + <$n>::one()
//     };
// }

#[macro_export]
macro_rules! increment {
    ($n:expr) => {
        $n = $n.checked_add(&T::one())?;
    };
}

#[macro_export]
macro_rules! big {
    ($seq: expr) => {
        $seq.into_iter().map(|x| BigInt::from(x)).collect()
    };
}

#[macro_export]
macro_rules! one_row {
    ($seq: expr, $skip: expr, $take: expr, $formatter:literal, $sep:literal) => {
        let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take)); // better to use fully qualified forms in macros
        let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!($formatter, x)), $sep);
        println!("{} {}..{}\n{}\n", stringify!($seq), $skip, $skip+$take, s);
    };
}

#[macro_export]
macro_rules! print_values {
    ($($sequence: expr, $skip: expr, $take: expr);+;) => {
        #[cfg(test)]
        #[ignore = "visualization"]
        #[test]
        fn print_values() {
            $(
                crate::one_row!($sequence, $skip, $take, "{}", ", ");
            )+
        }
    };
    ($name:ident, formatter $formatter:literal, sep $sep:literal; $($sequence: expr, $skip: expr, $take: expr);+;) => {
        #[cfg(test)]
        #[ignore = "visualization"]
        #[test]
        fn $name() {
            $(
                crate::one_row!($sequence, $skip, $take, $formatter, $sep);
            )+
        }
    };
}

#[macro_export]
macro_rules! check_iteration_times {
    ($($seq: expr, $skip: expr);+ $(;)?) => {
        #[cfg(test)]
        #[ignore = "visualization"]
        #[test]
        fn check_times() {
            $(
                let t = std::time::Instant::now();
                let mut s = $seq;
                for _ in 0..($skip) {
                    s.next();
                };
                let n = s.next().unwrap();
                let elapsed = t.elapsed();
                println!("{} {} -> {:?}\nduration: {:?}", stringify!($seq), $skip, n, elapsed);
            )+
        }
    };

}

#[macro_export]
macro_rules! check_sequences {
    ($($seq: expr, $skip: expr, $take: expr, $data: expr);+ $(;)?) => {
        #[cfg(test)]
        #[test]
        fn check_sequences() {
            $(
                let expected = $data.map(|x| num::BigInt::from(x)).to_vec();
                let calculated = itertools::Itertools::collect_vec($seq.skip($skip).take($take).map(|x| num::BigInt::from(x)));
                if expected != calculated {
                    panic!("failure to agree for {}\nexpected:   {:?}\ncalculated: {:?}", stringify!($seq), expected, calculated);
                }
            )+
        }
    };
}
