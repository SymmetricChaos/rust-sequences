pub struct Lindenmayer {
    string: String,
    transition: Box<dyn Fn(char) -> String>,
}

impl Lindenmayer {
    pub fn new<T>(init: String, transition: T) -> Self
    where
        T: Fn(char) -> String + 'static,
    {
        Self {
            string: init,
            transition: Box::new(transition),
        }
    }
}

impl Iterator for Lindenmayer {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.string.clone();
        let mut next = String::with_capacity(self.string.len());
        for c in self.string.chars() {
            next.push_str(&(self.transition)(c));
        }
        self.string = next;
        Some(out)
    }
}

#[cfg(test)]
fn tree_system(x: char) -> String {
    match x {
        '0' => String::from("1[0]0"),
        '1' => String::from("11"),
        _ => String::from(x),
    }
}

crate::print_values!(
    Lindenmayer::new(String::from("0"), tree_system),
    0, 4;
);
