/// Compositions of n that have k parts.
pub struct CompositionsNK {
    k: usize,
    comp: Vec<usize>,
    stop: bool,
}

impl CompositionsNK {
    pub fn new(n: usize, k: usize) -> Self {
        if n < k {
            panic!("n must be greater than k")
        }
        if k < 1 {
            panic!("k must be greater than 1")
        }
        let mut comp = vec![1; k - 1];
        comp.push(n - k + 1);
        Self {
            k,
            comp,
            stop: false,
        }
    }
}

impl Iterator for CompositionsNK {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        let out = self.comp.clone();

        // Find the rightmost 1 in the array
        let mut last = self.k - 1;

        while self.comp[last] == 1 {
            if last == 0 {
                self.stop = true;
                return Some(out);
            }
            last -= 1;
        }
        if last == 0 {
            self.stop = true;
            return Some(out);
        }

        let z = self.comp[last];
        self.comp[last - 1] += 1;
        self.comp[last] = 1;
        self.comp[self.k - 1] = z - 1;

        Some(out)
    }
}

pub struct CompositionsN {
    n: usize,
    k: usize,
    comps: CompositionsNK,
}

impl CompositionsN {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            k: n,
            comps: CompositionsNK::new(n, n),
        }
    }
}

impl Iterator for CompositionsN {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let comp = self.comps.next();
            if comp.is_some() {
                return comp;
            } else {
                self.k -= 1;
                if self.k == 0 {
                    return None;
                } else {
                    self.comps = CompositionsNK::new(self.n, self.k);
                    return self.comps.next();
                }
            }
        }
    }
}

crate::print_sequences!(
    CompositionsNK::new(5,1), 0, 10, "{:?}", "\n";
    CompositionsNK::new(5,2), 0, 10, "{:?}", "\n";
    CompositionsNK::new(5,3), 0, 10, "{:?}", "\n";
    CompositionsNK::new(5,4), 0, 10, "{:?}", "\n";
    CompositionsNK::new(5,5), 0, 10, "{:?}", "\n";
    CompositionsN::new(5), 0, 20, "{:?}", "\n";
);
