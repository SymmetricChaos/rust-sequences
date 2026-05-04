use itertools::Itertools;

pub struct EuclideanRhythm {
    s: Vec<u32>,
    idx: usize,
}

impl EuclideanRhythm {
    pub fn new(a: u32, b: u32) -> Self {
        let s = EuclideanRhythm::string(a, b)
            .chars()
            .map(|c| if c == '0' { 0 } else { 1 })
            .collect_vec();

        Self { s, idx: 0 }
    }

    pub fn string(a: u32, b: u32) -> String {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for _ in 0..a {
            left.push(String::from("1"));
        }
        for _ in a..b {
            right.push(String::from("0"));
        }

        loop {
            while right.len() > 0 {
                for i in 0..left.len() {
                    match right.pop() {
                        Some(s) => left[i].push_str(&s),
                        None => break,
                    }
                }
            }
            if left.len() == 1 {
                return left[0].clone();
            }
            let suffix = left.last().unwrap().clone();
            let x = left.iter().position(|x| x == &suffix).unwrap();
            right = left[x..].to_vec();
            left = left[..x].to_vec();
        }
    }
}

impl Iterator for EuclideanRhythm {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.s[self.idx];
        self.idx += 1;
        self.idx %= self.s.len();
        Some(out)
    }
}

crate::check_sequences!(
    EuclideanRhythm::new(5, 13), [1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0];
);
