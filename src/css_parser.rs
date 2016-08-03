use css_rules;

pub struct Parser {
    pub stylesheet: String,
    current: Vec<char>,
}

impl Parser {
    pub fn new(full_css: String) -> Parser {
        Parser {
            current: full_css.chars().collect(),
            stylesheet: full_css,
        }
    }

    pub fn parse_stylesheet(&mut self) -> css_rules::Stylesheet {
        css_rules::Stylesheet::new()
    }
}