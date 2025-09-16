/// An L-system defined by a function from char to Option<&'static str>
/// Constant symbols should return None and variables Some.
pub struct Lindenmayer {
    string: String,
    transition: Box<dyn Fn(char) -> Option<&'static str>>,
}

impl Lindenmayer {
    pub fn new<T>(init: &str, transition: T) -> Self
    where
        T: Fn(char) -> Option<&'static str> + 'static,
    {
        Self {
            string: init.to_string(),
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
    ($name:ident; $($a:literal => $b:literal),+ $(,)?) => {
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
l_system!(
    cantor_system;
    'a' => "aba",
    'b' => "bbb",
);

#[cfg(test)]
l_system!(
    algae_system;
    'a' => "ab",
    'b' => "a",
);

#[cfg(test)]
l_system!(
    peano_curve;
    'X' => "XFYFX+F+YFXFY-F-XFYFX",
    'Y' => "YFXFY-F-XFYFX+F+YFXFY",
);

#[cfg(test)]
l_system!(
    complex_bush;
    'V' => "[+++W][---W]YV",
    'W' => "+X[-W]Z",
    'X' => "-W[+X]Z",
    'Y' => "YZ",
    'Z' => "[-FFF][+FFF]F",
);

#[cfg(test)]
l_system!(
    thue_morse;
    '0' => "01",
    '1' => "10",
);

crate::print_values!(
    print_linenmayer, formatter "{}", sep "\n";
    Lindenmayer::new("0", tree_system), 0, 4;
    Lindenmayer::new("a", cantor_system), 0, 4;
    Lindenmayer::new("a", algae_system), 0, 7;
    Lindenmayer::new("X", peano_curve), 0, 3;
    Lindenmayer::new("VZFFF", complex_bush), 0, 4;
    Lindenmayer::new("0", thue_morse), 0, 6;
);
