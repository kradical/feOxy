//! The `css_parse` module parses css stylesheets into css rule datastructures.

use css::{Color, Declaration, Rule, Selector, SimpleSelector, Stylesheet, Value};

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
        let mut stylesheet = Stylesheet::default();

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
        let mut selectors = Vec::new();

        while self.chars.peek().map_or(false, |c| *c != '{') {
            let selector = self.parse_selector();

            if selector != Selector::default() {
                selectors.push(selector);
            }

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
        let mut sselector = SimpleSelector::default();
        let mut selector = Selector::default();

        self.consume_while(char::is_whitespace);

        sselector.tag_name = match self.chars.peek() {
            Some(&c) if is_valid_start_ident(c) => Some(self.parse_identifier()),
            _ => None,
        };

        let mut multiple_ids = false;
        while self.chars.peek().map_or(false, |c| *c != ',' && *c != '{' && !(*c).is_whitespace()) {
            match self.chars.peek() {
                Some(&c) if c =='#' =>  {
                    self.chars.next();
                    if sselector.id.is_some() || multiple_ids {
                        sselector.id = None;
                        multiple_ids = true;
                        self.parse_id();
                    } else {
                        sselector.id = self.parse_id();
                    }
                },
                Some(&c) if c == '.' => {
                    self.chars.next();
                    let class_name = self.parse_identifier();

                    if class_name != String::from("") {
                        sselector.classes.push(class_name);
                    }
                },
                _ => {
                    // consume invalid selector
                    self.consume_while(|c| c != ',' && c != '{');
                },
            }
        }

        if sselector != SimpleSelector::default() {
            selector.simple.push(sselector);
        }

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

    /// Wraps an identifier in an option
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

            let value = self.consume_while(|x| x != ';' && x != '\n' && x != '}');




            let declaration;
            // is a color value
            if property == "background-color" {
                let color = translate_color(&value);
                declaration = Declaration::new(property, Value::Color(color));
            } else {
                declaration = Declaration::new(property, Value::Other(value));
            }

            if self.chars.peek().map_or(false, |c| *c == ';') {
                declarations.push(declaration);
                self.chars.next();
            } else {
                self.consume_while(char::is_whitespace);
                if self.chars.peek().map_or(false, |c| *c == '}') {
                    declarations.push(declaration);
                }
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

/// Gets an rgba color struct from a string
fn translate_color(c: &str) -> Color {
    if c == "red" {
        Color { r: 1.0, g: 0.0, b: 0.0, a: 0.0 }
    } else if c == "green" {
        Color { r: 0.0, g: 1.0, b: 0.0, a: 0.0 }
    } else if c == "blue" {
        Color { r: 0.0, g: 0.0, b: 1.0, a: 0.0 }
    } else {
        Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }
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
//  -cascade
//  -specificity

/// Tests ----------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    use css::{Declaration, Rule, Selector, SimpleSelector, Stylesheet};

    /// Test a parser is constructed correctly.
    #[test]
    fn parser_regular() {
        let css = "p{lel:kek;}";
        let mut parser = CssParser::new(css);

        for character in String::from(css).chars() {
            assert_eq!(character, parser.chars.next().unwrap());
        }

        assert_eq!(None, parser.chars.peek());
    }

    /// Test an empty parser is constructed correctly.
    #[test]
    fn parser_empty() {
        let mut parser = CssParser::new("");

        for character in String::from("").chars() {
            assert_eq!(character, parser.chars.next().unwrap());
        }

        assert_eq!(None, parser.chars.peek());
    }

    /// Test an empty declaration
    #[test]
    fn declarations_empty() {
        let mut parser = CssParser::new("");
        assert_eq!(Vec::<Declaration>::new(), parser.parse_declarations());
    }

    /// Test the end of a declaration
    #[test]
    fn declarations_end() {
        let mut parser = CssParser::new("}");
        assert_eq!(Vec::<Declaration>::new(), parser.parse_declarations());
    }

    /// Test a regular declaration
    #[test]
    fn declarations_regular() {
        let mut parser = CssParser::new(
            "color:red;
             border-width: 1px;
             background-color: aqua
           }");
        let decl_col = Declaration::new(String::from("color"), String::from("red"));
        let decl_bw = Declaration::new(String::from("border-width"), String::from("1px"));
        let decl_bg_col = Declaration::new(String::from("background-color"), String::from("aqua"));

        let expected = vec![decl_col, decl_bw, decl_bg_col];
        assert_eq!(expected, parser.parse_declarations());
    }

    /// Test declaration: semi-colon missing
    #[test]
    fn declarations_invalid() {
        let mut parser = CssParser::new(
            "color:red;
             border-width: 1px
             background-color: aqua
           }");
        let decl_col = Declaration::new(String::from("color"), String::from("red"));
        let decl_bg_col = Declaration::new(String::from("background-color"), String::from("aqua"));

        let expected = vec![decl_col, decl_bg_col];
        assert_eq!(expected, parser.parse_declarations());
    }

    /// Test empty identifier
    #[test]
    fn identifier_empty() {
        let mut parser = CssParser::new("");
        assert_eq!(String::from(""), parser.parse_identifier());
    }

    /// Test end of identifier
    #[test]
    fn identifier_end() {
        let mut parser = CssParser::new(",");
        assert_eq!(String::from(""), parser.parse_identifier());
    }

    /// Test a regular identifier
    #[test]
    fn identifier_regular() {
        let mut parser = CssParser::new("identifier-one,");
        assert_eq!(String::from("identifier-one"), parser.parse_identifier());
    }

    /// Test a multi-section identifier
    #[test]
    fn identifier_long() {
        let mut parser = CssParser::new("identifier-one.class-one,");
        assert_eq!(String::from("identifier-one"), parser.parse_identifier());
    }

    /// Test an identifier beginning with -
    #[test]
    fn identifier_invalid() {
        let mut parser = CssParser::new("-identifier-one.class-one,");
        assert_eq!(String::from(""), parser.parse_identifier());
    }

    /// Test whitespace after the identifier
    #[test]
    fn identifier_whitespace() {
        let mut parser = CssParser::new("identifier p#id-one.class-one,");
        assert_eq!(String::from("identifier"), parser.parse_identifier());
    }

    /// Test an empty selector
    #[test]
    fn selector_empty() {
        let mut parser = CssParser::new("");
        assert_eq!(Selector::default(), parser.parse_selector());
    }

    /// Test selector parsing with a ,
    #[test]
    fn selector_end1() {
        let mut parser = CssParser::new(",");
        assert_eq!(Selector::default(), parser.parse_selector());
    }

    /// Test selector parsing with a {
    #[test]
    fn selector_end2() {
        let mut parser = CssParser::new("{");
        assert_eq!(Selector::default(), parser.parse_selector());
    }

    /// Test a regular selector
    #[test]
    fn selector_regular() {
        let mut parser = CssParser::new("p#id-one.class-one");

        let ex_ss = SimpleSelector::new(Some(String::from("p")), Some(String::from("id-one")), vec![String::from("class-one")]);
        let expected = Selector::new(vec![ex_ss], vec![]);

        assert_eq!(expected, parser.parse_selector());
    }

    /// Test multiple classes in a selector
    #[test]
    fn selector_multi_class() {
        let mut parser = CssParser::new(".class1.class2.class3");
        let ex_ss = SimpleSelector::new(None, None, vec![String::from("class1"), String::from("class2"), String::from("class3")]);
        let expected = Selector::new(vec![ex_ss], vec![]);
        assert_eq!(expected, parser.parse_selector());
    }

    /// Test multiple id's in a selector
    #[test]
    fn selector_multi_id() {
        let mut parser = CssParser::new("#id1#id2#id3");
        assert_eq!(Selector::default(), parser.parse_selector());
    }

    /// Test an invalid selector
    #[test]
    fn selector_invalid() {
        let mut parser = CssParser::new("-p#id-one.class-one");
        assert_eq!(Selector::default(), parser.parse_selector());
    }

    /// Test selectors parsing (comma seperated list)
    #[test]
    fn selectors_empty() {
        let mut parser = CssParser::new("");
        assert_eq!(Vec::<Selector>::new(), parser.parse_selectors());
    }

    /// Test selectors parsing (comma seperated list)
    #[test]
    fn selectors_end() {
        let mut parser = CssParser::new("{");
        assert_eq!(Vec::<Selector>::new(), parser.parse_selectors());
    }

    /// Test selectors parsing (comma seperated list)
    #[test]
    fn selectors_regular() {
        let mut parser = CssParser::new("tag1, #id1, .class1, _tag-2#id-2.class-2");

        let ssel1 = SimpleSelector::new(Some(String::from("tag1")), None, vec![]);
        let sel1 =  Selector::new(vec![ssel1], vec![]);

        let ssel2 = SimpleSelector::new(None, Some(String::from("id1")), vec![]);
        let sel2 =  Selector::new(vec![ssel2], vec![]);

        let ssel3 = SimpleSelector::new(None, None, vec![String::from("class1")]);
        let sel3 =  Selector::new(vec![ssel3], vec![]);

        let ssel4 = SimpleSelector::new(Some(String::from("_tag-2")), Some(String::from("id-2")), vec![String::from("class-2")]);
        let sel4 =  Selector::new(vec![ssel4], vec![]);


        assert_eq!(vec![sel1, sel2, sel3, sel4], parser.parse_selectors());
    }

    /// Test selectors parsing (comma seperated list one invalid)
    #[test]
    fn selectors_regular_one_invalid() {
        let mut parser = CssParser::new("tag1, #id1, .-class1, _tag-2#id-2.class-2");

        let ssel1 = SimpleSelector::new(Some(String::from("tag1")), None, vec![]);
        let sel1 =  Selector::new(vec![ssel1], vec![]);

        let ssel2 = SimpleSelector::new(None, Some(String::from("id1")), vec![]);
        let sel2 =  Selector::new(vec![ssel2], vec![]);

        let ssel3 = SimpleSelector::new(Some(String::from("_tag-2")), Some(String::from("id-2")), vec![String::from("class-2")]);
        let sel3 =  Selector::new(vec![ssel3], vec![]);


        assert_eq!(vec![sel1, sel2, sel3], parser.parse_selectors());
    }

    /// Test selectors parsing (comma seperated list all invalid)
    #[test]
    fn selectors_regular_all_invalid() {
        let mut parser = CssParser::new("-tag1, #-id1, .-class1, -_tag-2#id-2.class-2");
        assert_eq!(Vec::<Selector>::new(), parser.parse_selectors());
    }

    /// Test stylesheet parsing
    #[test]
    fn stylesheet_empty() {
        let mut parser = CssParser::new("");
        assert_eq!(Stylesheet::default(), parser.parse_stylesheet())
    }

    /// Test stylesheet parsing
    #[test]
    fn stylesheet_regular() {
        let mut parser = CssParser::new(
            "p {
                 color: red;
             }
             body#id1.class1,
             .class2.class3.class4 {
                 border: solid black 1px;
                 background-color: aqua
             }");
        let p_ss = SimpleSelector::new(Some(String::from("p")), None, vec![]);
        let p = Selector::new(vec![p_ss], vec![]);
        let p_decl = Declaration::new(String::from("color"), String::from("red"));
        let rule1 = Rule::new(vec![p], vec![p_decl]);

        let body_ss1 = SimpleSelector::new(Some(String::from("body")), Some(String::from("id1")), vec![String::from("class1")]);
        let body1 = Selector::new(vec![body_ss1], vec![]);
        let body_ss2 = SimpleSelector::new(None, None, vec![String::from("class2"), String::from("class3"), String::from("class4")]);
        let body2 = Selector::new(vec![body_ss2], vec![]);
        let body_decl1 = Declaration::new(String::from("border"), String::from("solid black 1px"));
        let body_decl2 = Declaration::new(String::from("background-color"), String::from("aqua"));
        let rule2 = Rule::new(vec![body1, body2], vec![body_decl1, body_decl2]);

        assert_eq!(Stylesheet::new(vec![rule1, rule2]), parser.parse_stylesheet())
    }
}
