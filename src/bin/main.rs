extern crate iron_oxide_browser;
use iron_oxide_browser::{command, css, css_parse, dom, html_parse, layout, render, style};

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
    viewport.content.width = 1024.0;
    viewport.content.height = 768.0;
    let layout_tree = layout::layout_tree(&style_tree_root, viewport);
    layout::pretty_print(&layout_tree, 0);

    let display_commands =  command::build_display_commands(&layout_tree);
    render::render_loop(&display_commands);
}

fn test_html() -> Vec<dom::Node> {
    let mut path = env::current_dir().unwrap();
    path.push("tests/parserTestFiles/blockTypeTest.html");

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
    path.push("tests/parserTestFiles/blockTypeTest.css");

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
// TODO change the rest of the code into library crates
