//! The css module provides a stylesheet datastructure for the css parser to use.

use std::fmt;

pub struct Stylesheet {
    pub rules: Vec<Rule>
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>
}

pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinators: Vec<char>
}

pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>
}

pub struct Declaration {
    pub property: String,
    pub value: String
}

impl Stylesheet {
    /// Constructs a new stylesheet.
    pub fn new() -> Stylesheet {
        Stylesheet {
            rules: Vec::new()
        }
    }
}
impl fmt::Debug for Stylesheet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rule_result = String::new();
        for rule in &self.rules {
            if rule_result.len() > 0 {
                rule_result.push_str("\n\n");
            }
            rule_result.push_str(&format!("{:?}", rule));
        }
        write!(f, "{}", rule_result)
    } 
}

impl Rule {
    /// Constructs a new Rule.
    ///
    /// s: All selectors the rule applies to.
    /// d: All declarations associated with the rule.
    pub fn new(s: Vec<Selector>, d: Vec<Declaration>) -> Rule {
        Rule {
            selectors: s,
            declarations: d,
        }
    }
}
impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sel_result = String::new();
        let mut decl_result = String::new();
        let tab = "    ";

        for selector in &self.selectors {
            if sel_result.len() > 0 {
                sel_result.push_str(", ");
            }
            sel_result.push_str(&format!("{:?}", selector));
        }

        for declaration in &self.declarations {
            decl_result.push_str(tab);
            decl_result.push_str(&format!("{:?}", declaration));
            decl_result.push('\n');
        }

        write!(f, "{} {{\n{}}}", sel_result, decl_result)
    }
}

impl Selector {
    /// Constructs a new Selector.
    ///
    /// SimpleSelectors can be combined with combinators into complex selectors.
    pub fn new() -> Selector {
        Selector {
            simple: Vec::new(),
            combinators: Vec:: new()
        }
    }
}
impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        for sel in &self.simple {
            if result.len() > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{:?}", sel));
        }

        write!(f, "{}", result)
    }
}

impl SimpleSelector {
    /// Constructs a new SimpleSelector.
    pub fn new() -> SimpleSelector {
        SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new()
        }
    }
}
impl fmt::Debug for SimpleSelector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        
        match self.tag_name {
            Some(ref t) => result.push_str(t),
            None => {} 
        }

        match self.id {
            Some(ref s) => {
                result.push('#');
                result.push_str(s);
            },
            None => {}
        }

        for class in &self.classes {
            result.push('.');
            result.push_str(class);
        }

        write!(f, "{}", result)
    }
}

impl Declaration {
    /// Constructs a new Declaration.
    ///
    /// p: Property name.
    /// v: Property value.
    pub fn new(p: String, v: String) -> Declaration {
        Declaration {
            property: p,
            value: v,
        }
    }
}
impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.property, self.value)
    }
}

// TODO
//  -add enum for css value types
//  -add creation logic for css value types
//  -write tests

/// Tests ----------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    /// Test
    #[test]
    fn it_works() {

    }
}