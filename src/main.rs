mod dom;
mod parser;

use std::env;
use std::fs::File;
use std::io::{Read, BufReader};

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("src/exampleHtml/ex3.html");

    let mut file_reader = match File::open(&path) {
        Ok(f) => BufReader::new(f),
        Err(e) => panic!("file: {}\nerror: {}", path.display(), e)
    };

    let mut html_input = String::new(); 
    file_reader.read_to_string(&mut html_input).unwrap();

    let nodes = parser::Parser::new(html_input).parse_nodes();
    
    for node in nodes.iter() {
        dom::pretty_print(node, 0);    
    }
}
