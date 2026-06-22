use std::cell::LazyCell;

use crate::Number;
use itertools::Itertools;
use num::traits::Euclid;
use regex::{Captures, Regex};

const INTEGERS: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"[0-9]+").unwrap());
const MAIN_TERMS: LazyCell<Regex> =
    LazyCell::new(|| Regex::new(r"(?: |^)([^ ]+)(?: \+|$)").unwrap());

fn base_string(n: Number, b: Number, sp: bool) -> String {
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
    if sp {
        terms.into_iter().rev().join(" + ")
    } else {
        terms.into_iter().rev().join("+")
    }
}

fn _hereditary_base_string_inner(n: Number, b: Number, sp: bool) -> String {
    let s = base_string(n, b, sp);

    let replacement = |caps: &Captures| -> String {
        let k: i64 = caps[0].parse().unwrap();

        if k > b {
            format!("({})", _hereditary_base_string_inner(k, b, false))
        } else {
            format!("{k}")
        }
    };

    if INTEGERS.find(&s).is_some() {
        return INTEGERS.replace_all(&s, &replacement).to_string();
    }

    s
}

pub fn hereditary_base_string(n: Number, b: Number) -> String {
    _hereditary_base_string_inner(n, b, true)
}

pub fn increase_herediary_base(s: String, b: Number) -> String {
    let replacement = |caps: &Captures| -> String {
        let k: i64 = caps[0].parse().unwrap();

        if k == b {
            format!("{}", k + 1)
        } else {
            format!("{k}")
        }
    };

    if INTEGERS.find(&s).is_some() {
        return INTEGERS.replace_all(&s, &replacement).to_string();
    }

    s
}

pub fn goodstein_step(s: String, b: Number) -> String {
    if &s == "0" || &s == "1" {
        return String::from("0");
    }

    let stepped = increase_herediary_base(s, b);

    let l = MAIN_TERMS.find_iter(&stepped).count();

    let final_term = MAIN_TERMS
        .captures_iter(&stepped)
        .last()
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    if final_term == "1" {
        return MAIN_TERMS
            .captures_iter(&stepped)
            .take(l - 1)
            .map(|c| c.get(1).unwrap().as_str())
            .join(" + ");
    }

    todo!()
}

#[cfg(test)]
mod base_test {
    use super::*;

    #[test]
    #[ignore = "visual test"]
    fn simple_base_test() {
        let n = 187;
        let b = 3;

        println!("{}", base_string(n, b, true));
        println!("{}", hereditary_base_string(n, b));
        println!("{}", goodstein_step(hereditary_base_string(n, b), b));
    }
}
