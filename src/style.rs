//! The `style` module takes a dom tree and stylesheet and constructs a style tree.

use dom::{Node, ElementData, NodeType};
use css::{Selector, Stylesheet};

use std::collections::HashMap;
use std::{fmt, str};

type PropertyMap<'a> = HashMap<&'a str, &'a str>;

pub struct StyledNode<'a> {
    node: &'a Node,
    styles: PropertyMap<'a>,
    pub children: Vec<StyledNode<'a>>
}

pub enum Display {
    Block,
    Inline,
    None
}

impl<'a> StyledNode<'a> {
    /// Constructs a new StyledNode
    ///
    /// node: The current dom node being styled.
    /// ss: The stylesheet being applied.
    pub fn new(node: &'a Node, ss: &'a Stylesheet) -> StyledNode<'a> {
        // recursively make a styletree without any styles
        // then apply rules to the tree 
        let mut style_children = Vec::new();

        for child in &node.children {
            match child.node_type {
                NodeType::Element(_) => style_children.push(StyledNode::new(&child, ss)),
                _ => {}
            }
        }

        StyledNode {
            node: node,
            styles: match node.node_type {
                NodeType::Element(ref e) => StyledNode::get_styles(e, ss),
                _ => PropertyMap::new()
            },
            children: style_children
        }
    }

    /// Return the styles of the current node.
    ///
    /// elem: The current node's element data.
    /// ss: The current stylesheet being applied.
    fn get_styles(elem: &'a ElementData, ss: &'a Stylesheet) -> PropertyMap<'a> {
        let mut styles = PropertyMap::new();

        for rule in &ss.rules {
            for selector in &rule.selectors {
                if selector_matches(elem, &selector) {
                    for decl in &rule.declarations {
                        styles.insert(&decl.property, &decl.value);
                    }
                    break;
                }
            }
        }

        styles
    }

    /// Return a style property for the current node.
    ///
    /// name: the property name to return the value of.
    pub fn value(&self, name: &str) -> Option<&&str> {
        self.styles.get(name)
    }

    /// Return the value of display property of the current node. 
    pub fn get_display(&self) -> Display {
        match self.value("display") {
            Some(s) => match *s {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline
            },
            None => Display::Inline
        }
    }

    /// Return a style property for the current node or a default value.
    ///
    /// name: the property name to return the value of.
    /// default: the value to return if None is found.
    pub fn value_or<T>(&self, name: &str, default: T) -> T where T: str::FromStr {
        match self.value(name) {
            Some(v) => v.parse().unwrap_or(default),
            None => default,
        }
    }
}
impl<'a> fmt::Debug for StyledNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.node, self.styles)
    }
}

/// Utility to check if a selector matches a dom node.
///
/// elem: The element data of the dom node to match.
/// sel: The selector to match.
fn selector_matches(elem: &ElementData, sel: &Selector) -> bool {
    for simple in &sel.simple {
        let mut sel_match = true;

        match simple.tag_name {
            Some(ref t) => {
                if *t != elem.tag_name {
                    continue;
                }
            },
            None => {}
        };

        match elem.get_id() {
            Some(i) => {
                match simple.id {
                    Some(ref id) => {
                        if *i != *id {
                            continue;
                        }
                    },
                    None => {}
                }
            },
            None => {
                match simple.id {
                    Some(_) => { continue; },
                    _ => {}
                }
            }
        }

        let elem_classes = elem.get_classes();

        for class in &simple.classes {
            sel_match &= elem_classes.contains::<str>(class);
        }

        if sel_match {
            return true;
        }
    }

    false
}

/// Print a styled node and it's descendents with indentation
///
/// n: The node of the style tree to print.
/// indent_size: the amount to indent the current node.
/// 
/// To pretty_print the full style tree pass the root node and 0
pub fn pretty_print(n: &StyledNode, indent_size: usize) {
    let indent = (0..indent_size).map(|_| " ").collect::<String>();

    println!("{}{:?}", indent, n);

    for child in n.children.iter() {
        pretty_print(&child, indent_size + 2);
    }
}

//TODO 
//  -make things case insensitive.
//  -parse element attributes into styles
//  -parse <style> elements into style sheets
//  -computed values
//  -inheritance
//  -write tests
//  -support multiple stylesheets

/// Tests ----------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /// Test
    #[test]
    fn it_works() {

    }
}