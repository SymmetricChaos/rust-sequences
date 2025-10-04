use itertools::Itertools;

// https://github.com/shawroger/combination/blob/master/src/internal.rs
fn c(starter: &Vec<usize>, group: usize) -> Vec<(Vec<usize>, usize)> {
    let starter_len = starter.len();
    let mut res = vec![];

    if starter_len < group || group < 1 {
        return res;
    }

    if group == 1 {
        return (0..starter_len).map(|i| (vec![starter[i]], i)).collect();
    }

    c(starter, group - 1).into_iter().for_each(|(arr, index)| {
        ((index + 1)..starter_len).into_iter().for_each(|i| {
            res.push(([arr.clone(), vec![starter[i]]].concat(), i));
        })
    });

    res
}

pub struct Combinations<T> {
    arr: Vec<T>,
    combi_idxs: Box<dyn Iterator<Item = Vec<usize>>>,
}

impl<T: Clone> Combinations<T> {
    pub fn new(arr: Vec<T>, size: usize) -> Self {
        Self {
            combi_idxs: Box::new(
                c(&(0..arr.len()).into_iter().collect_vec(), size)
                    .into_iter()
                    .map(|(v, _)| v),
            ),
            arr,
        }
    }
}

impl<T: Clone> Iterator for Combinations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let idxs = self.combi_idxs.next()?;
        let mut combi = Vec::new();
        for idx in idxs {
            combi.push(self.arr[idx].clone());
        }
        Some(combi)
    }
}

#[cfg(test)]
mod test {
    use num::BigInt;

    use super::*;

    #[test]
    fn combi() {
        let arr = (0..20).map(|x| BigInt::from(x)).collect();

        for c in Combinations::new(arr, 10) {
            println!("{:?}", c)
        }
    }
}
