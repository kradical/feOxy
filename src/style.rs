use dom::{Node, ElementData, NodeType};
use css::{Selector, Stylesheet};

use std::collections::HashMap;
use std::fmt;

type PropertyMap = HashMap<String, String>;

pub struct StyledNode<'a> {
    node: &'a Node,
    styles: PropertyMap,
    children: Vec<StyledNode<'a>>
}

impl<'a> StyledNode<'a> {
    pub fn new(node: &'a Node, ss: &Stylesheet) -> StyledNode<'a> {
        // recursively make a styletree without any styles
        // then apply rules to the tree 
        let mut style_children = Vec::new();

        for child in &node.children {
            match child.node_type {
                NodeType::Element(ref e) => style_children.push(StyledNode::new(&child, ss)),
                _ => {}
            }
        }

        StyledNode {
            node: node,
            styles: match node.node_type {
                NodeType::Element(ref e) => get_styles(e, ss),
                _ => PropertyMap::new()
            },
            children: style_children
        }
    }

    fn value(&self, name: &str) -> Option<&String> {
        self.styles.get(name)
    }
}

impl<'a> fmt::Debug for StyledNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {:?}", self.node, self.styles)
    }
}

fn get_styles(elem: &ElementData, ss: &Stylesheet) -> PropertyMap {
    let mut styles = PropertyMap::new();

    for rule in &ss.rules {
        for selector in &rule.selectors {
            if selector_matches(elem, &selector) {
                for decl in &rule.declarations {
                    styles.insert(decl.property.clone(), decl.value.clone());
                }
                break;
            }
        }
    }

    styles
}

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