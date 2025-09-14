/// An L-system defined by a function from char to Option<&'static str>
/// Constant symbols should return None and variables Some.
pub struct Lindenmayer {
    string: String,
    transition: Box<dyn Fn(char) -> Option<&'static str>>,
}

impl Lindenmayer {
    pub fn new<T>(init: String, transition: T) -> Self
    where
        T: Fn(char) -> Option<&'static str> + 'static,
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
            if let Some(s) = (self.transition)(c) {
                next.push_str(s);
            } else {
                next.push(c);
            }
        }
        self.string = next;
        Some(out)
    }
}

#[macro_export]
macro_rules! l_system {
    ($name:ident; $($a:literal => $b:literal);+ $(;)?) => {
        fn $name(x: char) -> Option<&'static str> {
            match x {
                $(
                    $a => Some($b),
                )+
                _ => None,
            }
        }
    };
}

#[cfg(test)]
fn tree_system(x: char) -> Option<&'static str> {
    match x {
        '0' => Some("1[0]0"),
        '1' => Some("11"),
        _ => None,
    }
}

#[cfg(test)]
fn cantor_system(x: char) -> Option<&'static str> {
    match x {
        'a' => Some("aba"),
        'b' => Some("bbb"),
        _ => None,
    }
}

#[cfg(test)]
l_system!(algae_system;
    'a' => "ab";
    'b' => "a";
);

crate::print_values!(
    Lindenmayer::new(String::from("0"), tree_system), 0, 4;
    Lindenmayer::new(String::from("a"), cantor_system), 0, 4;
    Lindenmayer::new(String::from("a"), algae_system), 0, 4;
);
