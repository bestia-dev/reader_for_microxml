//! microxml_print_to_screen
//!
//! use cargo run in the folder /examples/microxml_print_to_screen/
//! `cargo run ../t1.html`
//! `cargo run ../t2.html`
//! `cargo run ../t3.xml`
//!

use std::fs;
use std::env;
use std::io::Read;
use std::process;

use reader_for_microxml::{ReaderForMicroXml, Token};

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
    read_and_print(&text);
}

/// load file
fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

/// read xml and write to screen
pub fn read_and_print(input: &str) {
    let reader_iterator = ReaderForMicroXml::new(input);
    println!("\n{}\n\n", input);
    for res_token in reader_iterator {
        match res_token {
            Ok(token) => match token {
                Token::StartElement(name) => {
                    println!("Start Element name=\"{}\"", name);
                }
                Token::Attribute(name, value) => {
                    println!("Attribute name=\"{}\" value=\"{}\"", name, value);
                }
                Token::TextNode(txt) => {
                    println!("Text \"{}\"", txt);
                }
                Token::Comment(txt) => {
                    println!("Comment \"{}\"", txt);
                }
                Token::EndElement(name) => {
                    println!("End Element name=\"{}\"", name);
                }
            },
            Err(error_msg) => println!("Error text=\"{}\"", error_msg),
        }
    }
}
