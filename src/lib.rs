pub mod factorial;
pub mod fibonacci;
pub mod integers;
pub mod lucas;
pub mod lucas_sequence;
pub mod multiplicative_recurrence;
pub mod naturals;
pub mod oblong;
pub mod pell;
pub mod polygonal;
pub mod powers;
pub mod triangular;

#[macro_export]
macro_rules! print_a_few {
    ($($seq: expr, $skip: expr, $take: expr);+ $(;)?) => {

    #[cfg(test)]
    mod print_a_few {
        #[ignore = "visual check"]
        #[test]
        fn print_a_few_multi() {
            $(
                let ns: Vec<num::BigInt> = $seq.skip($skip).take($take).collect();
                println!("{:?}", ns);
            )+

        }
    }
    };
}
