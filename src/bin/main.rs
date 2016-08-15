extern crate rusty_browser;
use rusty_browser::{dom, css, style, layout, html_parse, css_parse};

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
    let mut viewport = layout::Dimensions::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;
    let layout_tree = layout::layout_tree(&style_tree_root, viewport);
    layout::pretty_print(&layout_tree);
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

    let nodes = html_parse::HtmlParser::new(&html_input).parse_nodes();

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

    let stylesheet = css_parse::CssParser::new(&css_input).parse_stylesheet();

    stylesheet
}

// TODO change this into a binary crate consumer of the rest of the code
// TODO change the rest of the code into a library crate