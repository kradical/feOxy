use std::collections::{HashMap, HashSet};
use std::fmt;

pub type AttrMap = HashMap<String, String>;

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String)
}

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::Text(ref t)|NodeType::Comment(ref t) => write!(f, "{}", t),
            NodeType::Element(ref e) => write!(f, "{:?}", e)
        }
    }
}

pub struct ElementData {
    pub tag_name: String,
    attributes: AttrMap
}

impl ElementData {
    pub fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(s) => s.split(' ').collect(),
            None => HashSet::new()
        }
    } 
}

impl fmt::Debug for ElementData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut attributes_string = String::new();
        
        for (attr, value) in self.attributes.iter() {
            attributes_string.push_str(&format!(" {}=\"{}\"", attr, value));
        }
        write!(f, "<{}{}>", self.tag_name, attributes_string)
    }
}

pub fn text_node(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn element_node(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element( ElementData {
            tag_name: name,
            attributes: attrs,
        })
    }
}

pub fn comment_node(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Comment(data) }
}

pub fn pretty_print(n: &Node, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    match n.node_type {
        NodeType::Element(ref e) => println!("{}{:?}", indent, e),
        NodeType::Text(ref t) => println!("{}{}", indent, t),
        NodeType::Comment(ref c) => println!("{}<!--{}-->", indent, c),
    }

    for child in n.children.iter() {
        pretty_print(&child, indent_size + 2);
    }

    match n.node_type {
        NodeType::Element(ref e) => println!("{}</{}>", indent, e.tag_name),
        _ => {}
    }
}