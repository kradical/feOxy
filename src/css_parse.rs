use css;

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

    pub fn parse_stylesheet(&mut self) -> css::Stylesheet {
        let mut stylesheet = css::Stylesheet::new();

        while self.has_chars() {
            let selectors = self.parse_selectors();
            let styles = self.parse_declarations();

            println!("{:?} {:?}", selectors, styles);
        }

        css::Stylesheet::new()
    }

    fn parse_selectors(&mut self) -> Vec<css::Selector> {
        let mut selectors = Vec::<css::Selector>::new();

        while self.has_chars() && self.peek() != '{' {
            self.consume_while(char::is_whitespace);
            
            let sel_string = self.consume_while(|x| x != ',' && x != '{');
            println!("{}", sel_string);
            
            if self.has_chars() && self.peek() == ',' {
                self.consume();
            }
        }

        if self.has_chars() {
            self.consume();
        }

        selectors
    }

    fn parse_declarations(&mut self) -> Vec<css::Declaration> {
        let declarations = Vec::<css::Declaration>::new();

        while self.has_chars() && self.peek() != '}' {
            self.consume();
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