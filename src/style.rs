use dom::{Node, ElementData, pretty_print, NodeType};
use css::{Selector};

use std::collections::HashMap;
use std::fmt;

type PropertyMap = HashMap<String, String>;

pub struct StyledNode<'a> {
    node: &'a Node,
    styles: PropertyMap,
    children: Vec<StyledNode<'a>>
}

impl<'a> StyledNode<'a> {
    pub fn new(node: &Node) -> StyledNode {
        // recursively make a styletree without any styles
        // then apply rules to the tree 
        let mut style_children = Vec::new();

        for child in &node.children {
            match child.node_type {
                NodeType::Element(ref e) => style_children.push(StyledNode::new(&child)),
                _ => {}
            }
        }

        StyledNode {
            node: node,
            styles: PropertyMap::new(),
            children: style_children
        }
    }
}

impl<'a> fmt::Debug for StyledNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        pretty_print(self.node, 0);
        write!(f, "styles of {:?}: {:?}", self.node, self.styles)
    }
}

pub fn selector_matches(elem: &ElementData, sel: &Selector) -> bool {
    let mut sel_match = true;

    for simple in &sel.simple {
        sel_match |= match simple.tag_name {
            Some(ref t) => *t == elem.tag_name,
            None => true
        };

        sel_match |= match simple.id {
            Some(ref i) => i == elem.get_id().unwrap_or(i),
            None => true
        };

        let elem_classes = elem.get_classes();

        for class in &simple.classes {
            sel_match |= elem_classes.contains::<str>(class);
        }
    }

    sel_match
}