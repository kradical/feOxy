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

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RULE")
    }
}

struct Selector {
    tag_name: Option<String>,
    id: Option<String>,
    class: Vec<String>
}

struct Declaration {
    name: String,
    value: Value
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
        println!("{}", rule);
    }
}