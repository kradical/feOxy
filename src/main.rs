mod dom;
mod css;
mod html_parse;
mod css_parse;

use std::env;
use std::fs::File;
use std::io::{Read, BufReader};

fn main() {
    test_html();
    test_css();
}

fn test_html() {
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
}

fn test_css() {
    let mut path = env::current_dir().unwrap();
    path.push("src/parserTestFiles/ex1.css");

    let mut file_reader = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file: {}\nerror: {}", path.display(), e)
    };

    let mut css_input = String::new(); 
    file_reader.read_to_string(&mut css_input).unwrap();

    let stylesheet = css_parse::Parser::new(css_input).parse_stylesheet();
    
    css::pretty_print(&stylesheet);
}
