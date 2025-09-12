pub mod catalan;
pub mod collatz;
pub mod core;
pub mod evil;
pub mod farey;
pub mod fibonacci;
pub mod figurate;
pub mod harmonic;
pub mod lucas;
pub mod lucas_sequence;
pub mod odious;
pub mod partition;
pub mod pell;
pub mod playground;
pub mod smooth;
pub mod squarefree;
pub mod strings;
pub mod thue_morse;
pub mod triangles;
pub mod trig;
pub mod zeta;

#[macro_export]
macro_rules! big {
    ($seq: expr) => {
        $seq.into_iter().map(|x| BigInt::from(x)).collect()
    };
}

#[macro_export]
macro_rules! print_values {
    ($($seq: expr, $skip: expr, $take: expr);+ $(;)?) => {
        #[cfg(test)]
        #[ignore = "visualization"]
        #[test]
        fn print_values() {
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
        fn print_rows() {
            $(
                let ns = itertools::Itertools::collect_vec($seq.skip($skip).take($take)); // better to use fully qualified forms in macros
                let s = itertools::Itertools::join(&mut ns.into_iter().map(|x| format!("{:?}", x)),"\n");
                println!("{} {}..{}\n{}\n", stringify!($seq), $skip, $skip+$take, s);
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
