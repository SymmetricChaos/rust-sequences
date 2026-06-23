/// Rivest Cipher 4 (RC4)
///
/// ```text
/// key = [0]
/// 222, 24, 137, 65, 163, 55, 93, 58, 138, 6, 30, 103,87, 110, 146...
/// ```
pub struct Rc4 {
    arr: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4 {
    /// RC4 can be initalized from any sequence of up to 256 bytes.
    pub fn new(key: &[u8]) -> Self {
        assert!(key.len() > 0);
        let mut arr = [0; 256];
        // Set array to identity permutation
        for n in 0..256 {
            arr[n] = n as u8;
        }
        // Perform 256 swaps
        let mut j: u8 = 0;
        for (i, k) in (0..256).zip(key.iter().cycle()) {
            j = j.wrapping_add(arr[i]).wrapping_add(*k);
            arr.swap(i, j as usize)
        }
        Self { arr, i: 0, j: 0 }
    }
}

impl Iterator for Rc4 {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.i = self.i.wrapping_add(1);
        self.j = self.j.wrapping_add(self.arr[self.i as usize]);
        self.arr.swap(self.i.into(), self.j.into());
        let t = self.arr[self.i as usize].wrapping_add(self.arr[self.j as usize]);
        Some(self.arr[t as usize])
    }
}

crate::sample_sequences!(
    Rc4::new(&[0]);
);
