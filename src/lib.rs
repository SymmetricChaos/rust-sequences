pub mod additive;
pub mod arithmetic;
pub mod catalan;
pub mod factorial;
pub mod fibonacci;
pub mod figurate;
pub mod geometric;
pub mod integers;
pub mod lucas;
pub mod lucas_sequence;
pub mod naturals;
pub mod parity;
pub mod pell;
pub mod playground;
pub mod power;
pub mod transforms;

#[macro_export]
macro_rules! print_a_few {
    ($($seq: expr, $skip: expr, $take: expr);+ $(;)?) => {
        #[cfg(test)]
        #[ignore = "visualization"]
        #[test]
        fn print_a_few_multi() {
            $(
                let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take)); // better to use fully qualified forms in macros
                println!("{} {}..{}\n{:?}\n", stringify!($seq), $skip, $take, ns);
            )+
        }
    };

}

#[macro_export]
macro_rules! check_sequences {
    ($($seq: expr, $skip: expr, $take: expr, $data: expr);+ $(;)?) => {
        #[cfg(test)]
            #[test]
            fn check_equality() {
                $(
                    let expected = $data.map(|x| num::BigInt::from(x)).to_vec();
                    let calculated = itertools::Itertools::collect_vec($seq.skip($skip).take($take));
                    if expected != calculated {
                        panic!("failure to agree\nexpected:   {:?}\ncalculated: {:?}", expected, calculated);
                    }
                )+
            }

    };
}
