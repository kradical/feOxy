mod dom;
mod css;
mod html_parse;
mod css_parse;
mod style;
mod layout;

use std::env;
use std::fs::File;
use std::io::{Read, BufReader};

fn main() {
    let nodes = test_html();
    for node in nodes.iter() {
        dom::pretty_print(node, 0);    
    }
    let ref node = nodes[0];

    println!("");
    let ss = test_css();
    print!("{:?}", ss);

    println!("");
    let style_tree_root = style::StyledNode::new(&node, &ss);
    style::pretty_print(&style_tree_root, 0);

    
    println!("");
    let layout_tree = layout::LayoutBox::new(layout::BoxType::Anonymous);
    print!("{:?}", layout_tree);
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

    stylesheet
}
