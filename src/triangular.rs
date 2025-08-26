pub struct Triangular32 {
    val: u32,
    ctr: u32,
}

impl Default for Triangular32 {
    fn default() -> Self {
        Self { val: 0, ctr: 0 }
    }
}

impl Triangular32 {
    pub fn new() -> Self {
        Self { val: 0, ctr: 0 }
    }
}

impl Iterator for Triangular32 {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let out = self.val;
        self.ctr = self.ctr.checked_add(1)?;
        self.val = self.val.checked_add(self.ctr)?;
        Some(out)
    }
}

mod tests {
    use super::*;

    #[test]
    fn seq() {
        let x = Triangular32::new();
        for n in x.skip(10).take(10) {
            println!("{n}")
        }
    }
}
