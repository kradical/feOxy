use css::{Stylesheet, Selector, SimpleSelector, Declaration, create_rule, create_declaration};
use parse::Parser;

pub struct CssParser {
    css_chars: Vec<char>,
    current: usize,
}

impl Parser for CssParser {
    /// Returns if the string still has characters in it.
    fn has_chars(&self) -> bool {
        return self.css_chars.len() > self.current;
    }

    /// Returns the current first character without consuming it.
    fn peek(&self) -> Option<char> {
        if self.current < self.css_chars.len() {
            return Some(self.css_chars[self.current])
        }
        None
    }

    /// Consumes the first character and returns it.
    fn consume(&mut self) -> Option<char> {
        let top_char = self.peek();
        self.current += 1;
        top_char
    }

    fn consume_while<F>(&mut self, condition: F) -> String where F : Fn(char) -> bool {
        let mut result = String::new();
        while self.peek().map_or(false, |c| condition(c)) {
            // the check above guarentees there is a value to be consumed
            result.push(self.consume().unwrap());
        }

        result
    }
}

impl CssParser {
    pub fn new(full_css: &str) -> CssParser {
        CssParser {
            css_chars: full_css.chars().collect(),
            current: 0,
        }
    }

    pub fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut stylesheet = Stylesheet::new();

        while self.has_chars() {
            let selectors = self.parse_selectors();
            let styles = self.parse_declarations();
            let rule = create_rule(selectors, styles);

            stylesheet.rules.push(rule);
        }

        stylesheet
    }

    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::<Selector>::new();

        while self.peek().map_or(false, |c| c != '{') {
            let selector = self.parse_selector();
            selectors.push(selector);

            self.consume_while(char::is_whitespace);
            if self.peek().map_or(false, |c| c == ',') {
                self.consume();
            }
        }

        self.consume();
        selectors
    }

    fn parse_selector(&mut self) -> Selector {
        let mut sselector = SimpleSelector::new();
        let mut selector = Selector::new();

        self.consume_while(char::is_whitespace);
        
        sselector.tag_name = match self.peek() {
            Some(c) if c == '#' || c == '.' => None,
            Some(_) => Some(self.parse_identifier()),
            None => None,
        };

        while self.peek().map_or(false, |c| c != ',' && c != '{' && !c.is_whitespace()) {
            match self.peek() {
                Some(c) if c =='#' =>  {
                    self.consume();
                    sselector.id = self.parse_id();
                },
                Some(c) if c == '.' => {
                    self.consume();
                    sselector.classes.push(self.parse_identifier());
                },
                _ => panic!("INVALID STATE IN parse_selector"), // TODO handle css errors
            }
        }

        selector.simple.push(sselector);
        selector
    }

    fn parse_identifier(&mut self) -> String {
        let mut ident = String::new();

        self.peek().map_or((), |c| {
            if is_valid_start_ident(c) { 
                ident.push_str(&self.consume_while(is_valid_ident))
            }
        });

        ident
    }

    fn parse_id(&mut self) -> Option<String> {
        match &self.parse_identifier()[..] {
            "" => None,
            s @ _ => Some(s.to_string())
        }
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::<Declaration>::new();

        while self.peek().map_or(false, |c| c != '}') {
            self.consume_while(char::is_whitespace);

            let property = self.consume_while(|x| x != ':');

            self.consume();
            self.consume_while(char::is_whitespace);

            //TODO fix for correctness
            let value = self.consume_while(|x| x != ';' && x != '\n' && x != '}');
            let declaration = create_declaration(property, value);

            declarations.push(declaration);

            if self.peek().map_or(false, |c| c == ';') {
                self.consume();
            }
            self.consume_while(char::is_whitespace);
        }

        self.consume();
        declarations
    }
}

fn is_valid_ident(c: char) -> bool {
    is_valid_start_ident(c) || c.is_digit(10) || c == '-'
}

fn is_valid_start_ident(c: char) -> bool {
    is_letter(c) || is_non_ascii(c) || c == '_'
}

fn is_letter(c: char) -> bool {
    is_upper_letter(c) || is_lower_letter(c)
}

fn is_upper_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn is_lower_letter(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

fn is_non_ascii(c: char) -> bool {
    c >= '\u{0080}'
}

//TODO 
//  -deal with comments and escaping characters
//  -complex selectors
//  -counter instead of destroy vec elements
//  -cascade
//  -specificity

/// Tests ----------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /// Test a parser is constructed correctly.
    #[test]
    fn new_parser() {
        let css_str = "p{lel:kek;}";
        let parser = CssParser::new(css_str);

        let expected_chars = vec![ 'p', '{', 'l', 'e', 'l', ':', 'k', 'e', 'k', ';', '}' ];

        assert_eq!(parser.current, 0);
        assert_eq!(parser.css_chars, expected_chars);
    }
}