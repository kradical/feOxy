//! The `css_parse` module parses css stylesheets into css rule datastructures.

use css::{Declaration, Rule, Selector, SimpleSelector, Stylesheet};

use std::iter::Peekable;
use std::str::Chars;

pub struct CssParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> CssParser<'a> {
    /// Constructs a new CssParser.
    ///
    /// full_css: the complete css stylesheet to parse.
    pub fn new(full_css: &str) -> CssParser {
        CssParser { chars: full_css.chars().peekable() }
    }

    /// Entry point to parsing css, iterively parse css rules.
    pub fn parse_stylesheet(&mut self) -> Stylesheet {
        let mut stylesheet = Stylesheet::new();

        while self.chars.peek().is_some() {
            let selectors = self.parse_selectors();
            let styles = self.parse_declarations();
            let rule = Rule::new(selectors, styles);

            stylesheet.rules.push(rule);
        }

        stylesheet
    }

    /// Parse the selectors for a single rule.
    fn parse_selectors(&mut self) -> Vec<Selector> {
        let mut selectors = Vec::<Selector>::new();

        while self.chars.peek().map_or(false, |c| *c != '{') {
            let selector = self.parse_selector();
            selectors.push(selector);

            self.consume_while(char::is_whitespace);
            if self.chars.peek().map_or(false, |c| *c == ',') {
                self.chars.next();
            }
        }

        self.chars.next();
        selectors
    }

    /// Parse a single selector in a comma seperated list of selectors.
    fn parse_selector(&mut self) -> Selector {
        let mut sselector = SimpleSelector::new();
        let mut selector = Selector::new();

        self.consume_while(char::is_whitespace);
        
        sselector.tag_name = match self.chars.peek() {
            Some(&c) if c == '#' || c == '.' => None,
            Some(_) => Some(self.parse_identifier()),
            None => None,
        };

        while self.chars.peek().map_or(false, |c| *c != ',' && *c != '{' && !(*c).is_whitespace()) {
            match self.chars.peek() {
                Some(&c) if c =='#' =>  {
                    self.chars.next();
                    sselector.id = self.parse_id();
                },
                Some(&c) if c == '.' => {
                    self.chars.next();
                    sselector.classes.push(self.parse_identifier());
                },
                _ => panic!("INVALID STATE IN parse_selector"), // TODO handle css errors
            }
        }

        selector.simple.push(sselector);
        selector
    }

    /// Parse a css identifier.
    fn parse_identifier(&mut self) -> String {
        let mut ident = String::new();

        match self.chars.peek() {
            Some(&c) => {
                if is_valid_start_ident(c) { 
                    ident.push_str(&self.consume_while(is_valid_ident))
                }
            },
            None => {},
        }

        ident
    }

    /// Parse the id portion of a selector.
    fn parse_id(&mut self) -> Option<String> {
        match &self.parse_identifier()[..] {
            "" => None,
            s @ _ => Some(s.to_string())
        }
    }

    /// Parse all the declarations for a rule.
    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::<Declaration>::new();

        while self.chars.peek().map_or(false, |c| *c != '}') {
            self.consume_while(char::is_whitespace);

            let property = self.consume_while(|x| x != ':');

            self.chars.next();
            self.consume_while(char::is_whitespace);

            //TODO fix for correctness
            let value = self.consume_while(|x| x != ';' && x != '\n' && x != '}');
            let declaration = Declaration::new(property, value);

            declarations.push(declaration);

            if self.chars.peek().map_or(false, |c| *c == ';') {
                self.chars.next();
            }
            self.consume_while(char::is_whitespace);
        }

        self.chars.next();
        declarations
    }

    /// Consumes characters until condition is false or there are no more chars left.
    /// Returns a string of the consumed characters.
    fn consume_while<F>(&mut self, condition: F) -> String where F : Fn(char) -> bool {
        let mut result = String::new();
        while self.chars.peek().map_or(false, |c| condition(*c)) {
            // the check above guarentees there is a value to be consumed
            result.push(self.chars.next().unwrap());
        }

        result
    }
}

/// Returns true if the char is a valid for a css identifier.
fn is_valid_ident(c: char) -> bool {
    is_valid_start_ident(c) || c.is_digit(10) || c == '-'
}

/// Returns true if the char is a valid for the first char of a css identifier.
fn is_valid_start_ident(c: char) -> bool {
    is_letter(c) || is_non_ascii(c) || c == '_'
}

/// Returns true if the char is an ASCII letter.
fn is_letter(c: char) -> bool {
    is_upper_letter(c) || is_lower_letter(c)
}

/// Returns true if the char is an ASCII uppercase char.
fn is_upper_letter(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

/// Returns true if the char is an ASCII lowercase char.
fn is_lower_letter(c: char) -> bool {
    c >= 'a' && c <= 'z'
}

/// Returns true if the char is non-ascii.
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
    use std::iter::Peekable;
    use std::str::Chars;

    /// Test a parser is constructed correctly.
    #[test]
    fn new_parser() {
        let (parser, mut expected_chars) = test_parser("p{lel:kek;}");

        for character in parser.chars {
            assert_eq!(character, expected_chars.next().unwrap());
        }

        assert_eq!(None, expected_chars.peek());
    }

    /// Utility to return a parser for tests. 
    fn test_parser<'a>(mock_css: &'a str) -> (CssParser, Peekable<Chars<'a>>) {
        let parser = CssParser::new(mock_css);
        let expected_chars = mock_css.chars().peekable();
        (parser, expected_chars)
    }
}