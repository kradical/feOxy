use std::fmt;

pub struct Stylesheet {
    rules: Vec<Rule>
}

impl Stylesheet {
    pub fn new() -> Stylesheet {
        Stylesheet {
            rules: Vec::new()
        }
    }
}

struct Rule {
    selectors: Vec<Selector>,
    styles: Vec<Declaration>
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RULE")
    }
}

pub struct Selector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>
}

impl fmt::Debug for Selector {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "SELECTOR")
	}
}

pub struct Declaration {
    name: String,
    value: Value
}

impl fmt::Debug for Declaration {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "DECLARATION")
	}
}

enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color)
}

enum Unit {
    Px
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub fn pretty_print(s: &Stylesheet) {
    println!("printing rules");
    for rule in &s.rules {
        println!("{:?}", rule);
    }
}