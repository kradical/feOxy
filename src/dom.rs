//! The `dom` module provides a dom node datastructure for an html parser to use.

use std::collections::{HashMap, HashSet};
use std::fmt;

pub type AttrMap = HashMap<String, String>;

#[derive(PartialEq, Eq)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

#[derive(PartialEq, Eq)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
    Comment(String),
}

#[derive(PartialEq, Eq)]
pub struct ElementData {
    pub tag_name: String,
    attributes: AttrMap,
}

impl Node {
    /// Constructs a new Node of given NodeType with given children.
    ///
    /// node_data: content of the node
    /// children: child nodes
    pub fn new(node_data: NodeType, children: Vec<Node>) -> Node {
        Node {
            children: children,
            node_type: node_data,
        }
    }
}
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.node_type)
    }
}

impl ElementData {
    /// Constructs a new ElementData containing a tag_name and attributes.
    ///
    /// tag: the tagname for the element
    /// attrs: a map of the elements {name: value}
    pub fn new(tag: String, attrs: AttrMap) -> ElementData{
        ElementData {
            tag_name: tag,
            attributes: attrs,
        }
    }

    /// Returns an element's id
    pub fn get_id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    /// Returns an element's classes
    pub fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(s) => s.split(' ').collect(),
            None => HashSet::new(),
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

impl fmt::Debug for NodeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NodeType::Text(ref t)|NodeType::Comment(ref t) => write!(f, "{}", t),
            NodeType::Element(ref e) => write!(f, "{:?}", e),
        }
    }
}

/// Print a node and it's descendents with indentation
///
/// n: The node of the html tree to print;
/// indent_size: the amount to indent the current node
/// 
/// To pretty_print the full dom pass the root node and 0
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
        _ => {},
    }
}

/// Tests ----------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};
    use std::iter::FromIterator;

    /// Test a Node is constructed properly.
    #[test]
    fn new_node() {
        let expected = Node { children: Vec::new(), node_type: NodeType::Text("test_type".to_string()) };
        let constructed = Node::new(NodeType::Text("test_type".to_string()), Vec::new());
        assert_eq!(expected, constructed);
    }

    /// Test an ElementData is constructed properly.
    #[test]
    fn new_elemdata() {
        let expected = ElementData { tag_name: "test_tag".to_string(), attributes: HashMap::new() };
        let constructed = ElementData::new("test_tag".to_string(), HashMap::new());
        assert_eq!(expected, constructed);
    }

    /// Test ElementData.get_id() returns id attribute.
    #[test]
    fn elemdata_get_id() {
        let mut attrs = HashMap::new();
        let tagname = String::from("has_id");
        let id_name = String::from("id");
        let id_value = String::from("identifier");

        attrs.insert(id_name, id_value);
        let elem = ElementData::new(tagname, attrs);

        let expected_str = String::from("identifier");
        let expected = Some(&expected_str);
        
        assert_eq!(expected, elem.get_id());
    }

    /// Test ElementData.get_id() returns none if there is no id.
    #[test]
    fn elemdata_get_id_empty() {
        let empty_map = HashMap::new();
        let tagname = String::from("no_attrs");
        let empty_elem = ElementData::new(tagname, empty_map);

        assert_eq!(None, empty_elem.get_id());
    }

    /// Test ElementData.get_id() returns the last id attribute specified.
    #[test]
    fn elemdata_get_id_multi() {
        let tagname = String::from("has_multi_ids");
        let id_value1 = String::from("identifier1");
        let id_value2 = String::from("identifier2");

        let mut attrs = HashMap::new();
        let id_name = String::from("id");
        attrs.insert(id_name, id_value1);

        let id_name = String::from("id");
        attrs.insert(id_name, id_value2);
        let elem = ElementData::new(tagname, attrs);

        let expected_str = String::from("identifier2");
        let expected = Some(&expected_str);
        
        assert_eq!(expected, elem.get_id());
    }

    /// Test ElementData.get_classes() returns a single class.
    #[test]
    fn elemdata_get_classes() {
        let tagname = String::from("has_class");
        let class_name = String::from("class");
        let class_value = String::from("a");
        
        let mut attrs = HashMap::new();
        attrs.insert(class_name, class_value);
        let elem = ElementData::new(tagname, attrs);

        let mut expected = HashSet::new();
        expected.insert("a");
        
        assert_eq!(expected, elem.get_classes());
    }

    /// Test ElementData.get_classes() returns an empty set if there is no class.
    #[test]
    fn elemdata_get_classes_empty() {
        let empty_map = HashMap::new();
        let tagname = String::from("no_attrs");
        let empty_elem = ElementData::new(tagname, empty_map);

        assert_eq!(HashSet::new(), empty_elem.get_classes());
    }

    /// Test ElementData.get_classes() returns a set of multiple classes.
    #[test]
    fn elemdata_get_classes_multi() {
        let tagname = String::from("has_classes");
        let class_name = String::from("class");
        let class_value = String::from("a b c top kekeroni");

        let mut attrs = HashMap::new();
        attrs.insert(class_name, class_value);
        let elem = ElementData::new(tagname, attrs);

        let expected_classes = vec!["a", "b", "c", "top", "kekeroni"];
        let expected = HashSet::from_iter(expected_classes);

        assert_eq!(expected, elem.get_classes());
    }
}
