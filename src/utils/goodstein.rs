use std::cell::LazyCell;

use crate::Number;
use itertools::Itertools;
use num::traits::Euclid;
use regex::{Captures, Regex};

const REG: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"[0-9]+").unwrap());

fn base_string(n: Number, b: Number) -> String {
    let mut terms = Vec::new();
    assert!(n.is_positive());
    assert!(b.is_positive());
    let mut n = n;
    let mut p = 0;
    while n != 0 {
        let (q, r) = n.div_rem_euclid(&b);
        if r != 0 {
            if r == 1 {
                if p == 0 {
                    terms.push(format!("{r}"));
                } else if p == 1 {
                    terms.push(format!("{b}"));
                } else {
                    terms.push(format!("{b}^{p}"));
                }
            } else {
                if p == 0 {
                    terms.push(format!("{r}"));
                } else if p == 1 {
                    terms.push(format!("{r}*{b}"));
                } else {
                    terms.push(format!("{r}*{b}^{p}"));
                }
            }
        }
        n = q;
        p += 1;
    }
    terms.into_iter().rev().join(" + ")
}

pub fn hereditary_base_string(n: Number, b: Number) -> String {
    let s = base_string(n, b);

    let replacement = |caps: &Captures| -> String {
        let k: i64 = caps[0].parse().unwrap();

        if k > b {
            format!("({})", hereditary_base_string(k, b))
        } else {
            format!("{k}")
        }
    };

    if REG.find(&s).is_some() {
        return REG.replace_all(&s, &replacement).to_string();
    }

    s
}

#[cfg(test)]
mod base_test {
    use super::*;

    #[test]
    fn simple_base_test() {
        println!("{}", base_string(1020, 2));

        println!("{}", hereditary_base_string(1020, 2));
    }
}
