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
pub mod parity;
pub mod pell;
pub mod polygonal;
pub mod power;
pub mod recurrence;
pub mod square;
pub mod tetrahedral;
pub mod transforms;
pub mod triangular;

#[macro_export]
macro_rules! print_a_few {
    ($($seq: expr, $skip: expr, $take: expr);+ $(;)?) => {
        #[cfg(test)]
        mod print_a_few {
            use super::*;
            // #[ignore = "visual check"]
            #[test]
            fn print_a_few_multi() {
                $(
                    let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take)); // better to use fully qualified forms in macros
                    println!("{} {}..{}\n{:?}\n", stringify!($seq), $skip, $take, ns);
                )+
            }
        }
    };

    ($mod_name: ident; $($seq: expr, $skip: expr, $take: expr);+ $(;)?) => {
        #[cfg(test)]
        mod $mod_name {
            use super::*;
            // #[ignore = "visual check"]
            #[test]
            fn print_a_few_multi() {
                $(
                    let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take)); // better to use fully qualified forms in macros
                    println!("{} {}..{}\n{:?}\n", stringify!($seq), $skip, $take, ns);
                )+
            }
        }
    };
}
