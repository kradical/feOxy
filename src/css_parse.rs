use css::{Stylesheet, Selector, SimpleSelector, Declaration, create_rule, 
    create_declaration};

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
            let selector = self.parse_selector();
            selectors.push(selector);

            self.consume_while(char::is_whitespace);
            if self.has_chars() && self.peek() == ',' {
                self.consume();
            }
        }

        if self.has_chars() {
            self.consume();
        }

        selectors
    }

    fn parse_selector(&mut self) -> Selector {
        let mut sselector = SimpleSelector::new();
        let mut selector = Selector::new();

        self.consume_while(char::is_whitespace);
        
        if self.has_chars() {
            sselector.tag_name = match self.peek() {
                '#'|'.' => None,
                _ => Some(self.parse_ident())
            }
        }

        while self.has_chars() && self.peek() != ',' && self.peek() != '{' && !self.peek().is_whitespace() {
            match self.peek() {
                '#' =>  {
                    self.consume();
                    sselector.id = self.parse_id()
                },
                '.' => {
                    self.consume();
                    sselector.classes.push(self.parse_ident())
                },
                _ => panic!("SOMEHOW INVALID STATE IN parse_selector")
            }
        }

        selector.simple.push(sselector);

        selector
    }

    fn parse_ident(&mut self) -> String {
        let mut ident = String::new();

        if self.has_chars() && is_valid_start_ident(self.peek()) {
            ident.push_str(&self.consume_while(is_valid_ident));
        }

        ident
    }

    fn parse_id(&mut self) -> Option<String> {
        match &self.parse_ident()[..] {
            "" => None,
            s @ _ => Some(s.to_string())
        }
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

//TODO deal with comments and escaping characters