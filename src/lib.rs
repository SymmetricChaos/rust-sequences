pub mod automata;
pub mod core;
pub mod figurate;
pub mod utils;

pub mod abundance;
pub mod abundant;
pub mod ackermann_sets;
pub mod algebraic;
pub mod arithmetic_derivative;
pub mod automorphic;
pub mod bell;
pub mod catalan;
pub mod collatz;
pub mod compositions;
pub mod compositions_weak;
pub mod deficient;
pub mod derangement;
pub mod digital_product;
pub mod digital_sum;
pub mod ducci;
pub mod eulers_number;
pub mod evil;
pub mod factoradic;
pub mod farey;
pub mod fibonacci;
pub mod gray;
pub mod harmonic;
pub mod hypotenuse;
pub mod jordan_polya;
pub mod leonardo;
pub mod look_and_say;
pub mod lucas;
pub mod lucas_sequence;
pub mod mobius;
pub mod odious;
pub mod padic_valuation;
pub mod partition;
pub mod pell;
pub mod phythagorean;
pub mod pi;
pub mod playground;
pub mod powerful;
pub mod rado_pairs;
pub mod recaman;
pub mod repint;
pub mod rowland;
pub mod smooth;
pub mod sorted_pairs;
pub mod squarefree;
pub mod sylverster;
pub mod thue_morse;
pub mod trig;
pub mod weyl;
pub mod zeta;

#[macro_export]
macro_rules! one_row {
    ($seq:expr, $take:literal; $($args:tt)*) => {
        let ns = itertools::Itertools::collect_vec($seq.take($take));
        let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!("{}", x)), ", ");
        println!("{} {}..{}\n{}\n", stringify!($seq), 0, $take, s);
        crate::one_row!($($args)*)
    };
    ($seq:expr, $skip:literal, $take:literal; $($args:tt)*) => {
        let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take));
        let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!("{}", x)), ", ");
        println!("{} {}..{}\n{}\n", stringify!($seq), $skip, $skip+$take, s);
        crate::one_row!($($args)*)
    };
    ($seq:expr, $take:literal, $format:literal, $sep:literal; $($args:tt)*) => {
        let ns = itertools::Itertools::collect_vec($seq.take($take));
        let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!($format, x)), $sep);
        println!("{} {}..{}\n{}\n", stringify!($seq), 0, $take, s);
        crate::one_row!($($args)*)
    };
    ($seq:expr, $skip:literal, $take:literal, $format:literal, $sep:literal; $($args:tt)*) => {
        let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take));
        let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!($format, x)), $sep);
        println!("{} {}..{}\n{}\n", stringify!($seq), $skip, $skip+$take, s);
        crate::one_row!($($args)*)
    };
    () => {}
}

#[macro_export]
macro_rules! print_sequences {
    ($name:ident; $($args:tt)*) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            crate::one_row!($($args)*);
        }
    };
    ($($args:tt)*) => {
        #[cfg(test)]
        #[test]
        fn print_sequences() {
            crate::one_row!($($args)*);
        }
    };
}

#[macro_export]
macro_rules! check_iteration_times {
    ($($seq:expr, $take:literal);+ $(;)?) => {
        #[cfg(test)]
        #[test]
        fn check_times() {
            $(
                let t = std::time::Instant::now();
                let mut s = $seq;
                for _ in 0..($take) {
                    s.next();
                };
                let n = s.next().unwrap();
                let elapsed = t.elapsed();
                println!("{} {} -> {:?}\nduration: {:?}\n", stringify!($seq), $take, n, elapsed);
            )+
        }
    };

}

#[macro_export]
macro_rules! check_sequences {
    // Start from the beginning of the sequence and take as many terms as needed to match the test length.
    ($($seq:expr, $data:expr);+ $(;)?) => {
        #[cfg(test)]
        #[test]
        fn check_sequences() {
            $(
                let expected = $data.map(|x| x.to_string()).to_vec();
                let calculated = itertools::Itertools::collect_vec($seq.take(expected.len()).map(|x| x.to_string()));
                if expected != calculated {
                    panic!("failure to agree for {}\nexpected:   {:?}\ncalculated: {:?}", stringify!($seq), expected, calculated);
                }
            )+
        }
    };
    ($($seq:expr, $skip:expr, $data:expr);+ $(;)?) => {
        #[cfg(test)]
        #[test]
        fn check_sequences() {
            $(
                let expected = $data.map(|x| x.to_string()).to_vec();
                let calculated = itertools::Itertools::collect_vec($seq.skip($skip).take(expected.len()).map(|x| x.to_string()));
                if expected != calculated {
                    panic!("failure to agree for {}\nexpected:   {:?}\ncalculated: {:?}", stringify!($seq), expected, calculated);
                }
            )+
        }
    };
}
