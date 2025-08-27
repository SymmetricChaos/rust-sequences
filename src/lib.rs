pub mod arithmetic;
pub mod factorial;
pub mod fibonacci;
pub mod geometric;
pub mod integers;
pub mod iterated_function;
pub mod lucas;
pub mod lucas_sequence;
pub mod naturals;
pub mod oblong;
pub mod pell;
pub mod polygonal;
pub mod power;
pub mod recurrence;
pub mod triangular;

#[macro_export]
macro_rules! print_a_few {
    ($($seq: expr, $skip: expr, $take: expr);+ $(;)?) => {

    #[cfg(test)]
    mod print_a_few {
        #[ignore = "visual check"]
        #[test]
        fn print_a_few_multi() {
            use super::*;
            use itertools::Itertools;
            $(
                let ns = $seq.skip($skip).take($take).collect_vec();
                println!("{} {}..{}\n{:?}\n", stringify!($seq), $skip, $take, ns);
            )+

        }
    }
    };
}
