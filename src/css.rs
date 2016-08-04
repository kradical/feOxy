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
    selectors: Vec<Selector>,
    declarations: Vec<Declaration>
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
    sel: String
}

impl fmt::Debug for Selector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.sel)
    }
}

pub struct Declaration {
    property: String,
    value: String
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

pub fn create_selector(sel: String) -> Selector {
    Selector {
        sel: sel
    }
}

pub fn create_declaration(prop: String, val: String) -> Declaration {
    Declaration {
        property: prop,
        value: val
    }
}