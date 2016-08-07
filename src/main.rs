mod dom;
mod css;
mod html_parse;
mod css_parse;
mod style;

use std::env;
use std::fs::File;
use std::io::{Read, BufReader};

fn main() {
    let nodes = test_html();
    let ref node = nodes[0];

    println!("");
    let ss = test_css();

    println!("");
    let style_tree_root = style::StyledNode::new(&node, &ss);

    style::pretty_print(&style_tree_root, 0);
}

fn test_html() -> Vec<dom::Node> {
    let mut path = env::current_dir().unwrap();
    path.push("src/parserTestFiles/ex3.html");

    let mut file_reader = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file: {}\nerror: {}", path.display(), e)
    };

    let mut html_input = String::new(); 
    file_reader.read_to_string(&mut html_input).unwrap();

    let nodes = html_parse::Parser::new(html_input).parse_nodes();
    
    for node in nodes.iter() {
        dom::pretty_print(node, 0);    
    }

    nodes
}

fn test_css() -> css::Stylesheet {
    let mut path = env::current_dir().unwrap();
    path.push("src/parserTestFiles/ex1.css");

    let mut file_reader = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file: {}\nerror: {}", path.display(), e)
    };

    let mut css_input = String::new(); 
    file_reader.read_to_string(&mut css_input).unwrap();

    let stylesheet = css_parse::Parser::new(css_input).parse_stylesheet();
    
    print!("{:?}", stylesheet);

    stylesheet
}
