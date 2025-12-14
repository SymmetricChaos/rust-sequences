/// Weak compositions of n that have k parts.
pub struct WeakCompositionsNK {
    k: usize,
    comp: Vec<usize>,
    stop: bool,
}

impl WeakCompositionsNK {
    pub fn new(n: usize, k: usize) -> Self {
        if n < k {
            panic!("n must be greater than k")
        }
        if k < 1 {
            panic!("k must be greater than 1")
        }
        let mut comp = vec![0; k - 1];
        comp.push(n);
        Self {
            k,
            comp,
            stop: false,
        }
    }
}

impl Iterator for WeakCompositionsNK {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stop {
            return None;
        }

        let out = self.comp.clone();

        // Find the rightmost 0 in the array
        let mut last = self.k - 1;

        while self.comp[last] == 0 {
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
        self.comp[last] = 0;
        self.comp[self.k - 1] = z - 1;

        Some(out)
    }
}

pub struct WeakCompositionsN {
    n: usize,
    k: usize,
    comps: WeakCompositionsNK,
}

impl WeakCompositionsN {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            k: n,
            comps: WeakCompositionsNK::new(n, n),
        }
    }
}

impl Iterator for WeakCompositionsN {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

crate::print_values!(
    print_arrays, formatter "{:?}", sep "\n";
    WeakCompositionsNK::new(5,3);
    // WeakCompositionsN::new(5), 0, 20;
);
