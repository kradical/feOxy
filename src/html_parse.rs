//! The `html_parse` module parses a string of html into dom node datastructures.

use dom::{AttrMap, ElementData, Node, NodeType};

use std::iter::Peekable;
use std::str::Chars;

pub struct HtmlParser<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> HtmlParser<'a> {
    /// Constructs a new HtmlParser.
    ///
    /// full_html: the complete html to parse.
    pub fn new(full_html: &str) -> HtmlParser {
        HtmlParser { chars: full_html.chars().peekable() }
    }

    /// Entry point to parsing html, recursively parses html nodes.
    /// TODO check tags match, deal with self closing tags
    pub fn parse_nodes(&mut self) -> Vec<Node> {
        let mut nodes = Vec::new();

        while self.chars.peek().is_some() {
            self.consume_while(char::is_whitespace);
            if self.chars.peek().map_or(false, |c| *c == '<') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '/') {
                    self.consume_while(|x| x != '>');
                    self.chars.next();
                    break;
                } else if self.chars.peek().map_or(false, |c| *c == '!') {
                    self.chars.next();
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

        while self.chars.peek().map_or(false, |c| *c != '<') {
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

        // ensure comment begins with <!--
        if self.chars.peek().map_or(false, |c| *c == '-') {
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '-') {
                self.chars.next();
            } else {
                self.consume_while(|c| c != '>');
                return Node::new(NodeType::Comment(comment_content), Vec::new());
            }
        } else {
            self.consume_while(|c| c != '>');
            return Node::new(NodeType::Comment(comment_content), Vec::new());
        }

        // comments beginning with > are invalid
        if self.chars.peek().map_or(false, |c| *c == '>') {
            self.chars.next();
            return Node::new(NodeType::Comment(comment_content), Vec::new());
        }

        // comments beginning with -> are invalid
        if self.chars.peek().map_or(false, |c| *c == '-') {
            self.chars.next();
            if self.chars.peek().map_or(false, |c| *c == '>') {
                self.chars.next();
                return Node::new(NodeType::Comment(comment_content), Vec::new());
            } else {
                comment_content.push('-');
            }
        }

        while self.chars.peek().is_some() {
            comment_content.push_str(&self.consume_while(|c| c != '<' && c != '-'));
            // check if comment contains <!-- and is invalid
            if self.chars.peek().map_or(false, |c| *c == '<') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '!') {
                    self.chars.next();
                    if self.chars.peek().map_or(false, |c| *c == '-') {
                        self.chars.next();
                        if self.chars.peek().map_or(false, |c| *c == '-') {
                            self.consume_while(|c| c != '>');
                            return Node::new(NodeType::Comment(String::from("")), Vec::new());
                        } else {
                            comment_content.push_str("<!-");
                        }
                    // <! ---> is an invalid sequence to end a comment
                    } else if self.chars.peek().map_or(false, |c| *c == ' ') {
                        self.chars.next();
                        if self.chars.peek().map_or(false, |c| *c == '-') {
                            self.chars.next();
                            if self.chars.peek().map_or(false, |c| *c == '-') {
                                self.chars.next();
                                if self.chars.peek().map_or(false, |c| *c == '-') {
                                    self.chars.next();
                                    if self.chars.peek().map_or(false, |c| *c == '>') {
                                        self.chars.next();
                                        return Node::new(NodeType::Comment(String::from("")), Vec::new());
                                    } else {
                                        comment_content.push_str("<! --");
                                    }
                                } else {
                                    comment_content.push_str("<! -");
                                }
                            } else {
                                comment_content.push_str("<! ");
                            }
                        }
                    } else {
                        comment_content.push_str("<!");
                    }
                } else {
                    comment_content.push('<');
                }
            } else if self.chars.peek().map_or(false, |c| *c == '-') {
                self.chars.next();
                if self.chars.peek().map_or(false, |c| *c == '-') {
                    self.chars.next();
                    if self.chars.peek().map_or(false, |c| *c == '>') {
                        self.chars.next();
                        break;
                    } else {
                        comment_content.push_str("--");
                    }
                } else {
                    comment_content.push('-');
                }
            }
        }

        Node::new(NodeType::Comment(comment_content), Vec::new())
    }

    /// Consume characters after a tagname until '>' and return a map.
    fn parse_attributes(&mut self) -> AttrMap {
        let mut attributes = AttrMap::new();

        while self.chars.peek().map_or(false, |c| *c != '>') {
            self.consume_while(char::is_whitespace);
            let name = self.consume_while(|c| is_valid_attr_name(c)).to_lowercase();
            self.consume_while(char::is_whitespace);

            let value = if self.chars.peek().map_or(false, |c| *c == '=') {
                self.chars.next(); // consume the '='
                self.consume_while(char::is_whitespace);
                let s = self.parse_attr_value();
                // cleans up aftere any invalid characters
                self.consume_while(|c| !c.is_whitespace() && c != '>');
                self.consume_while(char::is_whitespace);
                s
            } else {
                "".to_string()
            };
            attributes.insert(name, value);
        }
        self.chars.next(); // consume the '>' if it exists.

        attributes
    }

    /// Consume an attribute value and return it.
    fn parse_attr_value(&mut self) -> String {
        self.consume_while(char::is_whitespace);

        let result = match self.chars.peek() {
            Some(&c) if c == '"' || c == '\'' => {
                self.chars.next();
                let ret = self.consume_while(|x| x != c);
                self.chars.next(); // consume the quote
                ret
            },
            _ => self.consume_while(is_valid_attr_value),
        };

        result
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

/// Utility to check if a character can be used for an attribute name.
fn is_valid_attr_name(c: char) -> bool {
    !is_excluded_name(c) && !is_control(c)
}

fn is_control(ch: char) -> bool {
    match ch {
        '\u{007F}' => true,
        c if c >= '\u{0000}' && c <= '\u{001F}' => true,
        c if c >= '\u{0080}' && c <= '\u{009F}' => true,
        _ => false,
    }

}

fn is_excluded_name(c: char) -> bool {
    match c {
        ' '|'"'|'\''|'>'|'/'|'=' => true,
        _ => false,
    }
}

/// Utility to check if a character can be used for an attribute value.
/// TODO no ambiguous ampersand
fn is_valid_attr_value(c: char) -> bool {
    match c {
        ' '|'"'|'\''|'='|'<'|'>'|'`' => false,
        _ => true
    }
}

//TODO
//  -check and consume function that takes a condition
//  -script tags/link tags
//  -parse character references

/// Tests ----------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use super::is_control;
    use dom::{AttrMap, Node, NodeType};

    use std::iter::Peekable;
    use std::str::Chars;

    /// Test a parser is constructed correctly.
    #[test]
    fn new_parser() {
        let (parser, mut expected_chars) = test_parser("<p>lel</p>");

        for character in parser.chars {
            assert_eq!(character, expected_chars.next().unwrap());
        }

        assert_eq!(None, expected_chars.peek());
    }

    /// Test an empty attr value is parsed correctly.
    #[test]
    fn attr_value_empty() {
        let (mut parser, _) = test_parser("");
        assert_eq!("", parser.parse_attr_value());
    }

    /// Test an empty attr value is parsed correctly.
    #[test]
    fn attr_value_end() {
        let (mut parser, _) = test_parser(">");
        assert_eq!("", parser.parse_attr_value());
    }

    /// Test an regular attr value is parsed correctly.
    #[test]
    fn attr_value_reg() {
        let (mut parser, _) = test_parser("regularValue");
        assert_eq!("regularValue", parser.parse_attr_value());

        let (mut parser, _) = test_parser("regularValue>");
        assert_eq!("regularValue", parser.parse_attr_value());

        let (mut parser, _) = test_parser("regularValue ");
        assert_eq!("regularValue", parser.parse_attr_value());

        let (mut parser, _) = test_parser("regular<Value");
        assert_eq!("regular", parser.parse_attr_value());

        let (mut parser, _) = test_parser("regular'Value");
        assert_eq!("regular", parser.parse_attr_value());
    }

    /// Test an quoted attr value is parsed correctly.
    #[test]
    fn attr_value_quote() {
        let (mut parser, _) = test_parser("'regularValue'");
        assert_eq!("regularValue", parser.parse_attr_value());

        let (mut parser, _) = test_parser("\"regular'>< -_=Value\"");
        assert_eq!("regular'>< -_=Value", parser.parse_attr_value());

        let (mut parser, _) = test_parser("'regular\">< -_=Value'");
        assert_eq!("regular\">< -_=Value", parser.parse_attr_value());

        let (mut parser, _) = test_parser("\"regular\">< -_=Value\"");
        assert_eq!("regular", parser.parse_attr_value());

        let (mut parser, _) = test_parser("'regular'>< -_=Value'");
        assert_eq!("regular", parser.parse_attr_value());
    }

    /// Test empty attributes are parsed correctly.
    #[test]
    fn attrs_empty() {
        let (mut parser, _) = test_parser("");
        assert_eq!(AttrMap::new(), parser.parse_attributes());
    }

    /// Test end attributes are parsed correctly.
    #[test]
    fn attrs_end() {
        let (mut parser, _) = test_parser(">");
        assert_eq!(AttrMap::new(), parser.parse_attributes());
    }

    /// Test regular well formed attributes are parsed correctly.
    #[test]
    fn attrs_regular() {
        let (mut parser, _) = test_parser("name0 name1=value1 kek name2  ='value2' name3  = \"value3\"  ");
        let mut expected = AttrMap::new();
        expected.insert("name0".to_string(), "".to_string());
        expected.insert("kek".to_string(), "".to_string());
        expected.insert("name1".to_string(), "value1".to_string());
        expected.insert("name2".to_string(), "value2".to_string());
        expected.insert("name3".to_string(), "value3".to_string());

        assert_eq!(expected, parser.parse_attributes());
    }

    /// Test an invalid attribute.
    #[test]
    fn attrs_invalid() {
        let (mut parser, _) = test_parser("name0 name1=val'ue1 name2='va l ue2'");
        let mut expected = AttrMap::new();
        expected.insert("name0".to_string(), "".to_string());
        expected.insert("name1".to_string(), "val".to_string());
        expected.insert("name2".to_string(), "va l ue2".to_string());

        assert_eq!(expected, parser.parse_attributes());
    }

    /// Test case insensitivity for attr names and case sensitivity for attr values.
    #[test]
    fn attrs_case() {
        let (mut parser, _) = test_parser("NameZero NAMEone=VALUEone NAMETWO='VALUETWO' namethree=valuethree");
        let mut expected = AttrMap::new();
        expected.insert("namezero".to_string(), "".to_string());
        expected.insert("nameone".to_string(), "VALUEone".to_string());
        expected.insert("nametwo".to_string(), "VALUETWO".to_string());
        expected.insert("namethree".to_string(), "valuethree".to_string());

        assert_eq!(expected, parser.parse_attributes());
    }

    /// Test empty comment node.
    #[test]
    fn comment_empty() {
        let (mut parser, _) = test_parser("<!---->");
        let expected = Node::new(NodeType::Comment(String::from("")), Vec::new());

        assert_eq!(expected, parser.parse_comment_node());
    }

    /// Test end comment node.
    #[test]
    fn comment_end() {
        let (mut parser, _) = test_parser("-->");
        let expected = Node::new(NodeType::Comment(String::from("")), Vec::new());

        assert_eq!(expected, parser.parse_comment_node());
    }

    /// Test regular comment node.
    #[test]
    fn comment_regular() {
        let (mut parser, _) = test_parser("--Here is a comment \n '\"<>XD\"'-->");
        let comment_content = String::from("Here is a comment \n '\"<>XD\"'");
        let expected = Node::new(NodeType::Comment(comment_content), Vec::new());

        assert_eq!(expected, parser.parse_comment_node());
    }

    /// Test comment node that begins with >.
    #[test]
    fn comment_invalid1() {
        let (mut parser, _) = test_parser("-->Here is a comment \n '\"<>XD\"'-->");
        let expected = Node::new(NodeType::Comment(String::from("")), Vec::new());

        assert_eq!(expected, parser.parse_comment_node());
    }

    /// Test comment node that begins with ->.
    #[test]
    fn comment_invalid2() {
        let (mut parser, _) = test_parser("--->Here is a comment \n '\"<>XD\"'-->");
        let expected = Node::new(NodeType::Comment(String::from("")), Vec::new());

        assert_eq!(expected, parser.parse_comment_node());
    }

    /// Test comment node that conains <!--.
    #[test]
    fn comment_invalid3() {
        let (mut parser, _) = test_parser("--Here is a <!--comment \n '\"<>XD\"'-->");
        let expected = Node::new(NodeType::Comment(String::from("")), Vec::new());

        assert_eq!(expected, parser.parse_comment_node());
    }

    /// Test comment node that ends with <! -.
    #[test]
    fn comment_invalid4() {
        let (mut parser, _) = test_parser("--Here is a comment \n '\"<>XD\"'<! --->");
        let expected = Node::new(NodeType::Comment(String::from("")), Vec::new());

        assert_eq!(expected, parser.parse_comment_node());
    }

    /// Test if a character is a control character
    #[test]
    fn control_characters() {
        assert!(is_control('\u{0001}'));
        assert!(is_control('\u{007F}'));
        assert!(is_control('\u{0081}'));
        assert!(!is_control(' '));
    }

    /// Utility to return a parser for tests.
    fn test_parser<'a>(mock_html: &'a str) -> (HtmlParser, Peekable<Chars<'a>>) {
        let parser = HtmlParser::new(mock_html);
        let expected_chars = mock_html.chars().peekable();
        (parser, expected_chars)
    }
}
