//! microxml_tree
//!
//! use cargo run in the folder /examples/microxml_tree/
//! `cargo run ../t1.html`
//! `cargo run ../t2.html`
//! `cargo run ../t2err.html`
//! `cargo run ../t3.xml`
//!

use std::fs;
use std::env;
use std::io::Read;
use std::process;

use reader_for_microxml::{ReaderForMicroXml, Event};

#[derive(Debug)]
pub enum Node {
    Element(Element),
    Text(String),
}

#[derive(Debug)]
pub struct Element {
    name: String,
    attributes: Vec<Attribute>,
    ///Children
    nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct Attribute {
    name: String,
    value: String,
}

/// starting function
fn main() {
    println!("---microxml_print_to_screen---");
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage:    cargo run ../t2.html");
        process::exit(1);
    }
    let file_name = &args[1];
    println!("load file: {}", file_name);
    let text = load_file(file_name);

    let root_element = get_root_element(&text).expect("Error parsing xml.");
    println!("{:#?}", root_element);
}

/// load file
fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

/// read xml and return root_element or error
fn get_root_element(input: &str) -> Result<Element, String> {
    let mut pp = ReaderForMicroXml::new(input);
    println!("\n{}\n\n", input);

    let mut root_element;
    match pp.read_event() {
        Event::StartElement(name) => {
            root_element = Element {
                name: name.to_owned(),
                attributes: Vec::new(),
                nodes: Vec::new(),
            };
            //recursive function can return error
            match fill_element(&mut pp, &mut root_element) {
                Ok(()) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }
        _ => {
            //return error
            return Err("Error: no root element".to_owned());
        }
    }
    //return
    Ok(root_element)
}

/// recursive function to fill the tree with nodes
/// returns Result only because of errors
fn fill_element(pp: &mut ReaderForMicroXml, element: &mut Element) -> Result<(), String> {
    loop {
        match pp.read_event() {
            Event::StartElement(name) => {
                //make a child element and fill it (recursive)
                let mut child_element = Element {
                    name: name.to_owned(),
                    attributes: Vec::new(),
                    nodes: Vec::new(),
                };
                fill_element(pp, &mut child_element);
                element.nodes.push(Node::Element(child_element));
            }
            Event::Attribute(name, value) => {
                element.attributes.push(Attribute {
                    name: name.to_owned(),
                    value: value.to_owned(),
                });
            }
            Event::TextNode(txt) => {
                element.nodes.push(Node::Text(txt.to_owned()));
            }
            Event::EndElement(_name) => {
                return Ok(());
            }
            Event::Error(error_msg) => {
                return Err(format!("Error: {}", error_msg));
            }
            Event::Eof => {
                return Ok(());
            }
        }
    }
}
