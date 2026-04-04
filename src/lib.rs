pub mod automata;
pub mod core;
pub mod figurate;
pub mod utils;

pub mod a005243;
pub mod abelian_groups;
pub mod abundance;
pub mod abundant;
pub mod ackermann_sets;
pub mod algebraic;
pub mod antifibonacci;
pub mod arithmetic_derivative;
pub mod automorphic;
pub mod bell;
pub mod binomial_distribution;
pub mod catalan;
pub mod collatz;
pub mod compositions;
pub mod compositions_weak;
pub mod deficient;
pub mod derangement;
pub mod digital_product;
pub mod digital_sum;
pub mod dirichlet_convolution;
pub mod divisor;
pub mod ducci;
pub mod ecg;
pub mod eulers_number;
pub mod evil;
pub mod factoradic;
pub mod farey;
pub mod fermat;
pub mod fibonacci;
pub mod figure_figure;
pub mod fly_straight;
pub mod forest_fire;
pub mod golomb;
pub mod gray;
pub mod harmonic;
pub mod highly_composite;
pub mod highly_totient;
pub mod hofstadter_g;
pub mod hofstadter_h;
pub mod hofstadter_q;
pub mod hypotenuse;
pub mod jordan_polya;
pub mod juggler;
pub mod leonardo;
pub mod look_and_say;
pub mod lucas;
pub mod lucas_sequence;
pub mod mersenne;
pub mod mobius;
pub mod mountain;
pub mod narayana_general;
pub mod narayana_triangle;
pub mod narayanas_cows;
pub mod odd_part;
pub mod odious;
pub mod padic_valuation;
pub mod partition;
pub mod pascal;
pub mod pell;
pub mod perrin_padovan;
pub mod phythagorean;
pub mod pi;
pub mod playground;
pub mod powerful;
pub mod psuedoprime;
pub mod rado_pairs;
pub mod recaman;
pub mod regular_paperfolding;
pub mod repint;
pub mod rowland;
pub mod ruler;
pub mod semiprime;
pub mod smooth;
pub mod sorted_pairs;
pub mod squarefree;
pub mod stirling;
pub mod sylverster;
pub mod thue_morse;
pub mod totient;
pub mod trig;
pub mod unit;
pub mod weyl;
pub mod zeta;

#[macro_export]
macro_rules! print_row {
    // Take and use default formatting
    ($seq:expr, $take:literal; $($args:tt)*) => {
        let ns = itertools::Itertools::collect_vec($seq.take($take));
        let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!("{}", x)), ", ");
        println!("{} {}..{}\n{}\n", stringify!($seq), 0, $take, s);
        crate::print_row!($($args)*)
    };
    // Skip, then take and use default formatting
    ($seq:expr, skip $skip:literal, $take:literal; $($args:tt)*) => {
        let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take));
        let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!("{}", x)), ", ");
        println!("{} {}..{}\n{}\n", stringify!($seq), $skip, $skip+$take, s);
        crate::print_row!($($args)*)
    };
    // Take and use custom formatting
    ($seq:expr, $take:literal, $format:literal, $sep:literal; $($args:tt)*) => {
        let ns = itertools::Itertools::collect_vec($seq.take($take));
        let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!($format, x)), $sep);
        println!("{} {}..{}\n{}\n", stringify!($seq), 0, $take, s);
        crate::print_row!($($args)*)
    };
    // Skip, then take and use custom formatting
    ($seq:expr, skip $skip:literal, $take:literal, $format:literal, $sep:literal; $($args:tt)*) => {
        let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take));
        let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!($format, x)), $sep);
        println!("{} {}..{}\n{}\n", stringify!($seq), $skip, $skip+$take, s);
        crate::print_row!($($args)*)
    };
    () => {}
}

#[macro_export]
macro_rules! print_sequences {
    ($name:ident; $($args:tt)*) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            crate::print_row!($($args)*);
        }
    };
    ($($args:tt)*) => {
        #[cfg(test)]
        #[test]
        fn print_sequences() {
            crate::print_row!($($args)*);
        }
    };
}

#[macro_export]
macro_rules! check_row {
    ($seq:expr, $data:expr; $($args:tt)*) => {
        let expected = $data.map(|x| x.to_string()).to_vec();
        let calculated = itertools::Itertools::collect_vec($seq.take(expected.len()).map(|x| x.to_string()));
        if expected != calculated {
            panic!("failure to agree for {}\nexpected:   {:?}\ncalculated: {:?}", stringify!($seq), expected, calculated);
        }
        crate::check_row!($($args)*)
    };
    ($seq:expr, skip $skip:literal, $data:expr; $($args:tt)*) => {
        let expected = $data.map(|x| x.to_string()).to_vec();
        let calculated = itertools::Itertools::collect_vec($seq.skip($skip).take(expected.len()).map(|x| x.to_string()));
        if expected != calculated {
            panic!("failure to agree for {}\nexpected:   {:?}\ncalculated: {:?}", stringify!($seq), expected, calculated);
        }
        crate::check_row!($($args)*)
    };
    () => {}
}

#[macro_export]
macro_rules! check_sequences {
    ($name:ident; $($args:tt)*) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            crate::check_row!($($args)*);
        }
    };
    ($($args:tt)*) => {
        #[cfg(test)]
        #[test]
        fn check_sequences() {
            crate::check_row!($($args)*);
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
macro_rules! check_iteration_times_prog {
    ($($seq:expr, $take:expr);+ $(;)?) => {
        #[cfg(test)]
        #[test]
        fn check_times_prog() {
            $(
                println!("{}\n", stringify!($seq));

                let mut total_elapsed = std::time::Duration::new(0,0);
                let mut ctr = 0;
                let mut s = $seq;
                for r in $take {
                    let time = std::time::Instant::now();
                    while ctr < r {
                        s.next();
                        ctr += 1;
                    }
                    let n = s.next().unwrap();
                    total_elapsed += time.elapsed();
                    println!("{} terms\nn = {}\nduration: {:?}\n", r, n, total_elapsed);
                }
            )+
        }
    };
}
