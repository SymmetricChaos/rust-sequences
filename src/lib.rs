pub mod ackermann_sets;
pub mod additive;
pub mod catalan;
pub mod collatz;
pub mod core;
pub mod evil;
pub mod factorial;
pub mod fibonacci;
pub mod figurate;
pub mod harmonic;
pub mod lucas;
pub mod lucas_sequence;
pub mod odious;
pub mod pascal;
pub mod pell;
pub mod playground;
pub mod smooth;
pub mod thue_morse;

#[macro_export]
macro_rules! print_a_few {
    ($($seq: expr, $skip: expr, $take: expr);+ $(;)?) => {
        #[cfg(test)]
        #[ignore = "visualization"]
        #[test]
        fn print_a_few_multi() {
            $(
                let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take)); // better to use fully qualified forms in macros
                let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| x.to_string()),", ");
                println!("{} {}..{}\n{}\n", stringify!($seq), $skip, $skip+$take, s);
            )+
        }
    };
}

#[macro_export]
macro_rules! print_rows {
    ($($seq: expr, $skip: expr, $take: expr);+ $(;)?) => {
        #[cfg(test)]
        #[ignore = "visualization"]
        #[test]
        fn print_a_few_multi() {
            $(
                let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take)); // better to use fully qualified forms in macros
                let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!("{:?}", x)),"\n");
                println!("{} {}..{}\n{}\n", stringify!($seq), $skip, $skip+$take, s);
            )+
        }
    };

}

#[macro_export]
macro_rules! check_times {
    ($($seq: expr, $skip: expr);+ $(;)?) => {
        #[cfg(test)]
        #[ignore = "visualization"]
        #[test]
        fn print_a_few_multi() {

            $(
                let t = std::time::Instant::now();
                let ns = $seq.skip($skip).next().unwrap(); // better to use fully qualified forms in macros
                let elapsed = t.elapsed();
                println!("{} {} -> {:?}\nduration: {:?}", stringify!($seq), $skip, ns, elapsed);
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
                    let name = stringify!($seq);
                    let expected = $data.map(|x| num::BigInt::from(x)).to_vec();
                    let calculated = itertools::Itertools::collect_vec($seq.skip($skip).take($take).map(|x| num::BigInt::from(x)));
                    if expected != calculated {
                        panic!("failure to agree for {}\nexpected:   {:?}\ncalculated: {:?}", name, expected, calculated);
                    }
                )+
            }

    };
}

// #[macro_export]
// macro_rules! check_rational_sequences {
//     ($($seq: expr, $skip: expr, $take: expr, $data: expr);+ $(;)?) => {
//         #[cfg(test)]
//             #[test]
//             fn check_equality() {
//                 $(
//                     let expected = $data.map(|x| num::BigRational::from(x)).to_vec();
//                     let calculated = itertools::Itertools::collect_vec($seq.skip($skip).take($take));
//                     if expected != calculated {
//                         panic!("failure to agree\nexpected:   {:?}\ncalculated: {:?}", expected, calculated);
//                     }
//                 )+
//             }

//     };
// }
