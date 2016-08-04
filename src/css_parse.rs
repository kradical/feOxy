use css::{Stylesheet, Selector, Declaration, create_rule, create_selector, create_declaration};

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

        while self.has_chars() && self.peek() != '{' {
            self.consume_while(char::is_whitespace);
            //TODO fix selector parsing to be correct.
            let sel_string = self.consume_while(|x| x != ',' && x != '{');
            let selector = create_selector(sel_string);

            selectors.push(selector);
            
            if self.has_chars() && self.peek() == ',' {
                self.consume();
            }
        }

        if self.has_chars() {
            self.consume();
        }

        selectors
    }

    fn parse_declarations(&mut self) -> Vec<Declaration> {
        let mut declarations = Vec::<Declaration>::new();

        while self.has_chars() && self.peek() != '}' {
            self.consume_while(char::is_whitespace);
            
            let property = self.consume_while(|x| x != ':');
            
            if self.has_chars() {
                self.consume();
            }
            self.consume_while(char::is_whitespace);
            
            //TODO fix for correctness
            let value = self.consume_while(|x| x != ';' && x != '\n' && x != '}');
            let declaration = create_declaration(property, value);

            declarations.push(declaration);
            
            if self.has_chars() && self.peek() == ';' {
                self.consume();
            }
            self.consume_while(char::is_whitespace);
        }

        if self.has_chars() {
            self.consume();
        }

        declarations
    }

    fn has_chars(&self) -> bool {
        self.current.len() > 0
    }

    fn consume(&mut self) -> char {
        self.current.remove(0)
    }

    fn peek(&self) -> char {
        self.current[0]
    }

    fn consume_while<F>(&mut self, condition: F) -> String
        where F : Fn(char) -> bool {
            let mut result = String::new();
            while self.has_chars() && condition(self.peek()) {
                result.push(self.consume());
            }
            result
        }
}