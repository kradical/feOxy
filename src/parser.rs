use dom;

pub struct Parser {
    pub html_content: String,
    current_content: Vec<char>,
}

impl Parser {
    pub fn new(full_html: String) -> Parser {
        Parser {
            current_content: full_html.chars().collect(),
            html_content: full_html,
        }
    }

    pub fn parse_node(&mut self) {
        while self.has_chars() {
            self.consume_while(char::is_whitespace);
            
            if self.has_chars() && self.peek() == '<'{
                self.consume();
                if self.has_chars() && self.peek() == '/' {
                    // is a closing tag
                } else {
                    // is an opening tag
                    let tagname = self.consume_while(|x| x.is_digit(36));
                    let attributes = self.parse_attributes();

                    let elem = dom::element_node(tagname, attributes, Vec::new());
                    print!("{}\n", elem);
                    
                    self.consume_while(char::is_whitespace);
                    
                    if self.has_chars() && self.peek() == '>' {
                        //create dom element
                        self.consume();
                    }
                }
            }
            if self.has_chars() {
                self.consume();
            }
        }
    }

    // Enforces the string still has characters in it.
    fn has_chars(&mut self) -> bool {
        return self.current_content.len() > 0;
    }

    // Won't panic if only called after has_chars is tested.
    fn peek(&mut self) -> char {
        self.current_content[0]
    }

    // Won't panic if only called after has_chars is tested.
    fn consume(&mut self) -> char {
        self.current_content.remove(0)
    }

    // Won't panic if only called after has_chars is tested.
    fn consume_while<F>(&mut self, condition: F) -> String 
        where F : Fn(char) -> bool {
            let mut result = String::new();
            while self.has_chars() && condition(self.peek()) {
                result.push(self.consume());
            }
            result
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = dom::AttrMap::new();

        while self.has_chars() && self.peek() != '>' {
            self.consume_while(char::is_whitespace);
            let name = self.consume_while(is_valid_attr_name);
            self.consume_while(char::is_whitespace);

            if self.has_chars() {
                if self.peek() == '=' {
                    self.consume(); // consume equals sign
                    let value = self.parse_attr_value();
                    attributes.insert(name, value);
                } else if self.peek() == '>' || is_valid_attr_name(self.peek()) {
                    // new attribute hash with name -> ""
                    attributes.insert(name, "".to_string());
                } else {
                    // invalid attribute name consume until whitespace or end
                    self.consume_while(|x| !x.is_whitespace() || x != '>');
                }
            }

            self.consume_while(char::is_whitespace);
        }

        attributes
    }

    fn parse_attr_value(&mut self) -> String {
        self.consume_while(char::is_whitespace);
        let result = match self.consume() {
            c @ '"'| c @ '\'' => self.consume_while(|x| x != c && x != '>'),
            _ => self.consume_while(is_valid_attr_value)
        };
        if self.has_chars() {
            match self.peek() {
                '"'|'\'' => { self.consume(); },
                _ => { }
            }
        }
        result
    }
}

fn is_valid_attr_name(character: char) -> bool {
    // TODO deal with control characters
    // TODO  U+0020 SPACE, "tab" (U+0009), "LF" (U+000A), "FF" (U+000C), and "CR" (U+000D). instead of ' '
    match character {
        ' '|'"'|'\''|'>'|'/'|'=' => false,
        _ => true
    }
}

fn is_valid_attr_value(character: char) -> bool {
    // TODO no ambiguous ampersand
    match character {
        ' '|'"'|'\''|'<'|'>'|'`' => false,
        _ => true
    }
}

// need functions to:
//-read current char without consuming
//-peek (starts with)
//-check if input consumed
//-consume one char
//-consume while condition (returns consumed str)
//   -parse a node
//   -parse text node vs element node
//   -check if root node exists
//   -entry point (parse_nodes)


// pub fn parse_html(html_contents: &str) {
//     let mut inside_tag = false;
//     let mut tag = String::new();
//     let mut text = String::new();
//     let mut current_element = dom::element_node("root".to_string(), dom::AttrMap::new(), Vec::new());
//     let mut new_element = dom::element_node("child".to_string(), dom::AttrMap::new(), Vec::new());

//     current_element.children.push(new_element);
//     print!("{}\n\n", current_element.children[0]);

//     for character in html_contents.chars() {
//         if character == '>' {
//             parse_tag(&tag);
//             tag = String::new();
//             inside_tag = false;
//         } else if character == '<' {
//             parse_text(text);
//             text = String::new();
//             inside_tag = true;
//         } else if inside_tag {
//             tag.push(character);
//         } else {
//             text.push(character);
//         }
//     }
// }

// fn parse_tag(tag: &str) {
//     let mut first_word = true;
//     let mut attr_name = true;
//     let mut tagname = String::new();
//     let mut name_str = String::new();
//     let mut value_str = String::new();
//     let mut attributes = dom::AttrMap::new();    

//     for character in tag.chars() {
//         if character.is_whitespace() {
//             process_attr(&mut name_str, &mut value_str, &mut attributes);
//             attr_name = true;
//             first_word = false;
//             continue;
//         }

//         if character == '=' {
//             attr_name = false;
//             continue;
//         }

//         if first_word {
//             tagname.push(character);
//         } else if attr_name {
//             name_str.push(character);
//         } else {
//             value_str.push(character);
//         }
//     }
//     process_attr(&mut name_str, &mut value_str, &mut attributes);

//     let elem = dom::element_node(tagname, attributes, Vec::new());
//     print!("{}", elem);
// }

// fn parse_text(text: String) {
//     let mut all_space = true;
//     for character in text.chars() {
//         if !character.is_whitespace() {
//             all_space = false;
//             break;
//         }
//     }
//     if !all_space {
//         let text_node = dom::text_node(text);
//         print!("{}", text_node);       
//     }
// }

// fn process_attr(name: &mut String, value: &mut String, attr_map: &mut dom::AttrMap) {
//     if value.len() > 0 {
//         value.remove(0);
//         value.pop();   
//     }

//     if name.len() > 0 {
//         attr_map.insert(name.clone(), value.clone());
//         *name = String::new();
//         *value = String::new();
//     }
// }