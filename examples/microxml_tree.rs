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

use reader_for_microxml::{ReaderForMicroXml, Token};

#[derive(Debug)]
pub enum Node {
    Element(Element),
    Text(String),
    Comment(String),
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
    println!("---microxml_tree---");
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
    let mut reader_iterator = ReaderForMicroXml::new(input);
    println!("\n{}\n\n", input);

    let mut root_element = Element {
        name: "error no root".to_owned(),
        attributes: Vec::new(),
        nodes: Vec::new(),
    };
    let opt_result_token = reader_iterator.next();
    if let Some(result_token) = opt_result_token {
        match result_token {
            Ok(token) => {
                match token {
                    //only one root element
                    Token::StartElement(name) => {
                        root_element = Element {
                            name: name.to_owned(),
                            attributes: Vec::new(),
                            nodes: Vec::new(),
                        };
                        //recursive function can return error
                        match fill_element(&mut reader_iterator, &mut root_element) {
                            Ok(()) => {}
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    }
                    _ => {
                        //return error
                        return Err("Error: xml must start with a root element.".to_owned());
                    }
                }
            }
            Err(error_msg) => {
                // Err token
                return Err(error_msg.to_owned());
            }
        }
    } else {
        //opt is None. Should be eof.
    }
    //return
    Ok(root_element)
}

/// recursive function to fill the tree with nodes
/// returns Result only because of errors
fn fill_element(
    reader_iterator: &mut ReaderForMicroXml,
    element: &mut Element,
) -> Result<(), String> {
    loop {
        if let Some(result_token) = reader_iterator.next() {
            match result_token {
                Ok(token) => {
                    match token {
                        Token::StartElement(name) => {
                            //make a child element and fill it (recursive)
                            let mut child_element = Element {
                                name: name.to_owned(),
                                attributes: Vec::new(),
                                nodes: Vec::new(),
                            };
                            fill_element(reader_iterator, &mut child_element).unwrap();
                            element.nodes.push(Node::Element(child_element));
                        }
                        Token::Attribute(name, value) => {
                            element.attributes.push(Attribute {
                                name: name.to_owned(),
                                value: value.to_owned(),
                            });
                        }
                        Token::TextNode(txt) => {
                            element.nodes.push(Node::Text(txt.to_owned()));
                        }
                        Token::Comment(txt) => {
                            element.nodes.push(Node::Comment(txt.to_owned()));
                        }
                        Token::EndElement(_name) => {
                            break;
                        }
                    }
                }
                Err(error_msg) => return Err(error_msg.to_owned()),
            }
        } else {
            println!("Token is None");
            break;
        }
    }
    Ok(())
}
