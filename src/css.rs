//! The `css` module provides a stylesheet datastructure for the css parser to use.

use std::fmt;
use std::default::Default;

#[derive(PartialEq)]
pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

#[derive(PartialEq)]
pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>,
}

#[derive(PartialEq, Eq)]
pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinators: Vec<char>,
}

#[derive(PartialEq, Eq)]
pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>,
}

#[derive(PartialEq)]
pub struct Declaration {
    pub property: String,
    pub value: Value,
}

#[derive(PartialEq)]
pub enum Value {
    Color(Color),
    Other(String),
}

#[derive(PartialEq, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Stylesheet {
    /// Constructs a new stylesheet.
    pub fn new(r: Vec<Rule>) -> Stylesheet {
        Stylesheet { rules: r }
    }
}
impl Default for Stylesheet {
    fn default() -> Self {
        Stylesheet { rules: Vec::new() }
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
impl Default for Rule {
    fn default() -> Self {
        Rule {
            selectors: Vec::new(),
            declarations: Vec::new(),
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
    pub fn new(s: Vec<SimpleSelector>, c: Vec<char>) -> Selector {
        Selector {
            simple: s,
            combinators: c,
        }
    }
}
impl Default for Selector {
    fn default() -> Self {
        Selector {
            simple: Vec::new(),
            combinators: Vec::new(),
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
    pub fn new(t: Option<String>, i: Option<String>, c: Vec<String>) -> SimpleSelector {
        SimpleSelector {
            tag_name: t,
            id: i,
            classes: c,
        }
    }
}
impl Default for SimpleSelector {
    fn default() -> Self {
        SimpleSelector {
            tag_name: None,
            id: None,
            classes: Vec::new(),
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
    pub fn new(p: String, v: Value) -> Declaration {
        Declaration {
            property: p,
            value: v,
        }
    }
}
impl Default for Declaration {
    fn default() -> Self {
        Declaration {
            property: String::from(""),
            value: Value::Other(String::from("")),
        }
    }
}
impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.property, self.value)
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Color(ref c) => write!(f, "{:?}", c),
            Value::Other(ref s) => write!(f, "{:?}", s),
        }
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r: {} g: {} b: {} a: {}", self.r, self.g, self.b, self.a)
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

    /// Test a new stylesheet is constructed correctly.
    #[test]
    fn new_stylesheet() {
        let ss = Stylesheet::default();

        assert_eq!(ss.rules, vec![]);
    }

    /// Test a new rule is constructed correctly.
    #[test]
    fn new_rule() {
        let rule = Rule::new(vec![], vec![]);

        assert_eq!(rule.selectors, vec![]);
        assert_eq!(rule.declarations, vec![]);
    }

    /// Test a new selector is constructed correctly.
    #[test]
    fn new_selector() {
        let sel = Selector::default();

        assert_eq!(sel.simple, vec![]);
        assert_eq!(sel.combinators, vec![]);
    }

    /// Test a new simple selector is constructed correctly.
    #[test]
    fn new_simple() {
        let ss = SimpleSelector::default();
        let expected_classes: Vec<String> = vec![];

        assert_eq!(ss.tag_name, None);
        assert_eq!(ss.id, None);
        assert_eq!(ss.classes, expected_classes);
    }

    /// Test a new declaration is constructed correctly.
    #[test]
    fn new_declaration() {
        let decl = Declaration::default();

        assert_eq!(decl.property, "");
        assert_eq!(decl.value, "");
    }
}
