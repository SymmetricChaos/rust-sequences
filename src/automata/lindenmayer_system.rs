/// An L-system defined by a function from char to Option<&'static str>
/// Constant symbols should return None and variables Some.
pub struct Lindenmayer {
    transition: Box<dyn Fn(char) -> Option<&'static str>>,
}

impl Lindenmayer {
    pub fn new<T>(transition: T) -> Self
    where
        T: Fn(char) -> Option<&'static str> + 'static,
    {
        Self {
            transition: Box::new(transition),
        }
    }

    /// Run the automaton on an input.
    pub fn create_iter(&self, initital_string: &str) -> LindenmayerIter<'_> {
        LindenmayerIter {
            string: initital_string.to_string(),
            transition: &self.transition,
        }
    }
}

pub struct LindenmayerIter<'a> {
    string: String,
    transition: &'a Box<dyn Fn(char) -> Option<&'static str>>,
}

impl<'a> Iterator for LindenmayerIter<'a> {
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

/// Create a transition function that defines an L-system on strings. The transition for each variable symbol must be given. All other symbols are treated as constant symbols.
///
/// Example:
/// ```
/// l_system!(
///    tree;
///    '0' => "1[0]0"
///    '1' => "11"
/// );
/// ```
#[macro_export]
macro_rules! l_system {
    ($name:ident; $($a:literal => $b:literal)+) => {
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
mod tests {
    use super::*;

    // Lindenmayer's original algae system
    l_system!(
        algae;
        'a' => "ab"
        'b' => "a"
    );

    // Simple system featuring constant symbols
    l_system!(
        tree;
        '0' => "1[0]0"
        '1' => "11"
    );

    l_system!(
        cantor;
        'a' => "aba"
        'b' => "bbb"
    );

    l_system!(
        peano_curve;
        'X' => "XFYFX+F+YFXFY-F-XFYFX"
        'Y' => "YFXFY-F-XFYFX+F+YFXFY"
    );

    l_system!(
        complex_bush;
        'V' => "[+++W][---W]YV"
        'W' => "+X[-W]Z"
        'X' => "-W[+X]Z"
        'Y' => "YZ"
        'Z' => "[-FFF][+FFF]F"
    );

    l_system!(
        thue_morse;
        '0' => "01"
        '1' => "10"
    );

    l_system!(
        ternary_thue_morse;
        '0' => "1"
        '1' => "20"
        '2' => "210"
    );

    l_system!(
        period_doubling;
        '0' => "01"
        '1' => "00"
    );

    l_system!(
        fibonacci;
        '_' => "_|"
        '|' => "_"
    );

    l_system!(
        tribonacci;
        'a' => "ab"
        'b' => "ac"
        'c' => "a"
    );

    crate::print_sequences!(
        Lindenmayer::new(algae).create_iter("a"), 7, "{}", "\n";
        Lindenmayer::new(tree).create_iter("0"), 5, "{}", "\n";
        Lindenmayer::new(cantor).create_iter("a"), 5, "{}", "\n";
        Lindenmayer::new(peano_curve).create_iter("X"), 3, "{}", "\n";
        Lindenmayer::new(complex_bush).create_iter("VZFFF"), 4, "{}", "\n";
        Lindenmayer::new(thue_morse).create_iter("0"), 7, "{}", "\n";
        Lindenmayer::new(ternary_thue_morse).create_iter("0"), 7, "{}", "\n";
        Lindenmayer::new(period_doubling).create_iter("0"), 7, "{}", "\n";
        Lindenmayer::new(fibonacci).create_iter("_"), 7, "{}", "\n";
        Lindenmayer::new(tribonacci).create_iter("a"), 7, "{}", "\n";
    );
}
