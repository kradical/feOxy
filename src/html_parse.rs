use dom::{AttrMap, ElementData, Node, NodeType};

pub struct Parser {
    html_chars: Vec<char>,
    current: usize,
}

impl Parser {
    /// Constructs a new `html_parse::Parser`.
    ///
    /// full_html: the complete html to parse.
    pub fn new(full_html: &str) -> Parser {
        Parser {
            html_chars: full_html.chars().collect(),
            current: 0,
        }
    }

    /// Entry point to parsing, recursively parses html nodes.
    /// TODO check tags match, deal with self closing tags
    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while self.has_chars() {
            self.consume_while(char::is_whitespace);
            if self.peek().unwrap_or('_') == '<' {
                self.consume();
                if self.peek().unwrap_or('_') == '/' {
                    self.consume_while(|x| x != '>');
                    self.consume();
                    break;
                } else if self.peek().unwrap_or('_') == '!' {
                    self.consume();
                    nodes.push(self.parse_comment_node());
                } else {
                    nodes.push(self.parse_node());
                }
            } else {
                nodes.push(self.parse_text_node());
            }
        }
        nodes
    }

    /// Parse a single html node and recursively call parse_nodes on children.
    fn parse_node(&mut self) -> Node {
        // is an valid tagname
        let tagname = self.consume_while(|x| x.is_digit(36));
        let attributes = self.parse_attributes();

        let elem = ElementData::new(tagname, attributes);
        let children = self.parse_nodes();
        Node::new(NodeType::Element(elem), children)
    }

    /// Consume the text between '>' and '<' to produce a text node.
    /// TODO deal with control characters and U+0000
    fn parse_text_node(&mut self) -> Node {
        let mut text_content = String::new();

        while self.peek().unwrap_or('<') != '<' {
            let whitespace = self.consume_while(char::is_whitespace);
            if whitespace.len() > 0 {
                text_content.push(' ');
            }
            let text_part = self.consume_while(|x| !x.is_whitespace() && x != '<');
            text_content.push_str(&text_part);
        }
        Node::new(NodeType::Text(text_content), Vec::new())
    }

    /// Consume text between "<!--" and "-->" to produce a comment node.
    fn parse_comment_node(&mut self) -> Node {
        let mut comment_content = String::new();

        if self.peek().unwrap_or('_') == '-' {
            self.consume();
            if self.peek().unwrap_or('_') == '-' {
                self.consume();
                if self.peek().unwrap_or('_') == '>' {
                    // invalid comment format
                    self.consume();
                    return Node::new(NodeType::Comment(comment_content), Vec::new());
                } else if self.peek().unwrap_or('_') == '-' {
                    self.consume();
                    if self.peek().unwrap_or('_') == '>' {
                        // invalid comment format
                        self.consume();
                        return Node::new(NodeType::Comment(comment_content), Vec::new());
                    } else {
                        comment_content.push('-');
                    }
                }
                while self.has_chars() {
                    comment_content.push_str(&self.consume_while(|x| x != '-'));
                    if self.peek().unwrap_or('_') == '-' {
                        self.consume();
                        if self.peek().unwrap_or('_') == '-' {
                            self.consume_while(|x| x != '>');
                            self.consume();
                            break;
                        } else {
                            comment_content.push('-')
                        }
                    }
                }
            }
        }
        Node::new(NodeType::Comment(comment_content), Vec::new())
    }

    /// Returns if the string still has characters in it.
    fn has_chars(&self) -> bool {
        return self.html_chars.len() > self.current;
    }

    /// Returns the current first character without consuming it.
    fn peek(&self) -> Option<char> {
        if self.current < self.html_chars.len() {
            return Some(self.html_chars[self.current])
        }
        None
    }

    /// Consumes the first character and returns it.
    fn consume(&mut self) -> Option<char> {
        let top_char = self.peek();
        self.current += 1;
        top_char
    }

    /// Consumes characters until condition is false or the html_chars is empty.
    /// Returns a string of the consumed characters.
    fn consume_while<F>(&mut self, condition: F) -> String 
        where F : Fn(char) -> bool {
            let mut result = String::new();
            while match self.peek() {
                Some(c) => condition(c),
                None => false,
            } {
                // free to unwrap because the check above guarentees there is a value to be consumed
                result.push(self.consume().unwrap());
            }
            result
    }

    /// Consume characters after a tagname until '>' and return a map.
    /// TODO normalize caps
    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = AttrMap::new();

        while self.peek().unwrap_or('>') != '>' {
            self.consume_while(char::is_whitespace);
            let name = self.consume_while(is_valid_attr_name);
            self.consume_while(char::is_whitespace);

            if self.peek().unwrap_or('_') == '=' {
                self.consume(); // consume equals sign
                let value = self.parse_attr_value();
                attributes.insert(name, value);
            } else if self.peek().unwrap_or('_') == '>' || is_valid_attr_name(self.peek().unwrap_or(' ')) {
                // new attribute hash with name -> ""
                attributes.insert(name, "".to_string());
            } else {
                // invalid attribute name consume until whitespace or end
                self.consume_while(|x| !x.is_whitespace() || x != '>');
            }
            self.consume_while(char::is_whitespace);
        }

        if self.peek().unwrap_or('_') == '>' {
            self.consume();
        }

        attributes
    }

    /// Consume an attribute value (<tagname attrname=value>) and return it.
    /// TODO proper validation and error recovery
    fn parse_attr_value(&mut self) -> String {
        self.consume_while(char::is_whitespace);

        let result = match self.peek().unwrap_or('_') {
            c @ '"'| c @ '\'' => {
                self.consume();
                self.consume_while(|x| x != c && x != '>')
            },
            _ => self.consume_while(is_valid_attr_value),
        };

        match self.peek().unwrap_or('_') {
            '"'|'\'' => { self.consume(); },
            _ => {}
        }

        result
    }
}

/// Utility to check if a character can be used for an attribute name.
/// TODO deal with control characters
/// TODO  U+0020 SPACE, "tab" (U+0009), "LF" (U+000A), "FF" (U+000C), and "CR" (U+000D). instead of ' '
fn is_valid_attr_name(character: char) -> bool {
    match character {
        ' '|'"'|'\''|'>'|'/'|'=' => false,
        _ => true
    }
}

/// Utility to check if a character can be used for an attribute value.
/// TODO no ambiguous ampersand
fn is_valid_attr_value(character: char) -> bool {
    match character {
        ' '|'"'|'\''|'<'|'>'|'`' => false,
        _ => true
    }
}

//TODO 
//  -check and consume function that takes a condition
//  -parse text/comment nodes vs element node
//  -script tags/link tags
//  -parse character references
//  -use a counter instead of destroying an element of the vector each time.

/// Tests ----------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /// Test a parser is constructed correctly.
    #[test]
    fn new_parser() {
        let html_str = "<p>lel</p>";
        let parser = Parser::new(html_str);

        let expected_chars = vec![ '<', 'p', '>', 'l', 'e', 'l', '<', '/', 'p', '>' ];

        assert_eq!(parser.current, 0);
        assert_eq!(parser.html_chars, expected_chars);
    }

    /// Test a parser is constructed correctly.
    #[test]
    fn new_parser() {
        let html_str = "<p>lel</p>";
        let parser = Parser::new(html_str);

        let expected_chars = vec![ '<', 'p', '>', 'l', 'e', 'l', '<', '/', 'p', '>' ];

        assert_eq!(parser.current, 0);
        assert_eq!(parser.html_chars, expected_chars);
    }
}
