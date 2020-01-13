mod reader_for_microxml;

use reader_for_microxml::{ReaderForMicroXml, Event};
use std::fs;
use std::env;
use std::io::Read;
use std::process;

fn main() {
    println!("start main(){}-----------------------------------------------------------------------","");
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        println!("Usage:\n\tcargo run --example ast -- input.xml");
        process::exit(1);
    }

    let text = load_file(&args[1]);
    test_my_microxml(&text);
}

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

pub fn test_my_microxml(input: &str) {
    let mut pp = ReaderForMicroXml::new(input);
    //println!("ReaderForMicroXml::new(input) {}", "");
    loop {
        match pp.read_event() {
            Event::StartElement(name) => {
                println!("StartElement(name) .{}.", name);
            }
            Event::Attribute(name, value) => {
                println!("Attribute(name, value) .{}. .{}.", name, value);
            }
            Event::Nothing => {
                println!("Nothing .{}.", "");
            }
            Event::Text(txt) => {
                println!("Text(txt) .{}.", txt);
            }
            Event::EndElement(name) => {
                println!("EndElement(name) .{}.", name);
            }
            Event::Comment => {
                //comment is not data for MicroXml
                println!("Comment .{}.", "");
            }
            Event::Error(error_msg) => {
                println!("Error {}", error_msg);
                break;
            }
            Event::Eof => {
                println!("Eof {}", "");
                break;
            }
        }
    }
}
