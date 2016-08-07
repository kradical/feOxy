use std::fmt;

pub struct Stylesheet {
    pub rules: Vec<Rule>
}

impl Stylesheet {
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

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declaration>
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

pub struct Selector {
    pub simple: Vec<SimpleSelector>,
    pub combinators: Vec<char>
}

impl Selector {
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

pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub classes: Vec<String>
}

impl SimpleSelector {
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

pub struct Declaration {
    pub property: String,
    pub value: String
}

impl fmt::Debug for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.property, self.value)
    }
}

pub fn create_rule(sel: Vec<Selector>, decl: Vec<Declaration>) -> Rule {
    Rule {
        selectors: sel,
        declarations: decl
    }
}

pub fn create_declaration(prop: String, val: String) -> Declaration {
    Declaration {
        property: prop,
        value: val
    }
}