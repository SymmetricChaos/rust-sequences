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
pub mod powerful;

#[macro_export]
macro_rules! big {
    ($seq: expr) => {
        $seq.into_iter().map(|x| BigInt::from(x)).collect()
    };
}

#[macro_export]
macro_rules! one_row {
    // Assume the iterator is finite and take all elements.
    ($seq: expr, $formatter:literal, $sep:literal) => {
        let ns = itertools::Itertools::collect_vec($seq); // better to use fully qualified forms in macros
        let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!($formatter, x)), $sep);
        println!("{}\n{}\n", stringify!($seq), s);
    };
    // Assume the iterator is infinite. Skip zero or more elements then take a specified number.
    ($seq: expr, $skip: expr, $take: expr, $formatter:literal, $sep:literal) => {
        let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take)); // better to use fully qualified forms in macros
        let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!($formatter, x)), $sep);
        println!("{} {}..{}\n{}\n", stringify!($seq), $skip, $skip+$take, s);
    };
}

#[macro_export]
macro_rules! print_values {
    // Assume a finite sequence that needs simple formatting.
    ($($sequence: expr);+;) => {
        #[cfg(test)]
        #[test]
        fn print_values() {
            $(
                crate::one_row!($sequence, "{}", ", ");
            )+
        }
    };
    // Assume a finite sequence that needs special formatting.
    ($name:ident, formatter $formatter:literal, sep $sep:literal; $($sequence: expr);+;) => {
        #[cfg(test)]
        #[test]
        fn print_values() {
            $(
                crate::one_row!($sequence, $formatter, $sep);
            )+
        }
    };
    // Assume an infinite sequence that needs simple formatting.
    ($($sequence: expr, $skip: expr, $take: expr);+;) => {
        #[cfg(test)]
        #[test]
        fn print_values() {
            $(
                crate::one_row!($sequence, $skip, $take, "{}", ", ");
            )+
        }
    };
    // Assume an infinite sequence that needs special formatting.
    ($name:ident, formatter $formatter:literal, sep $sep:literal; $($sequence: expr, $skip: expr, $take: expr);+;) => {
        #[cfg(test)]
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
                println!("{} {} -> {:?}\nduration: {:?}\n", stringify!($seq), $skip, n, elapsed);
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
                let expected = $data.map(|x| x.to_string()).to_vec();
                let calculated = itertools::Itertools::collect_vec($seq.skip($skip).take($take).map(|x| x.to_string()));
                if expected != calculated {
                    panic!("failure to agree for {}\nexpected:   {:?}\ncalculated: {:?}", stringify!($seq), expected, calculated);
                }
            )+
        }
    };
}
