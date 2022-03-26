// region: lmake_md_to_doc_comments include README.md A //!
//! # reader for microXml
//!
//! **reader for microXml - the simplified subset of xml**  
//! ***[repo](https://github.com/LucianoBestia/reader_for_microxml); version: 2.0.2  date: 2021-01-13 authors: Luciano Bestia***  
//!
//!  [![crates.io](https://img.shields.io/crates/v/reader_for_microxml.svg)](https://crates.io/crates/reader_for_microxml) [![Documentation](https://docs.rs/reader_for_microxml/badge.svg)](https://docs.rs/reader_for_microxml/) [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/reader_for_microxml.svg)](https://web.crev.dev/rust-reviews/crate/reader_for_microxml/) [![RustActions](https://github.com/LucianoBestia/reader_for_microxml/workflows/rust/badge.svg)](https://github.com/LucianoBestia/reader_for_microxml/) [![latest doc](https://img.shields.io/badge/latest_docs-GitHub-orange.svg)](https://lucianobestia.github.io/reader_for_microxml/reader_for_microxml/index.html) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/LucianoBestia/reader_for_microxml/blob/main/LICENSE)
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-278-green.svg)](https://github.com/LucianoBestia/reader_for_microxml/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-208-blue.svg)](https://github.com/LucianoBestia/reader_for_microxml/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-64-purple.svg)](https://github.com/LucianoBestia/reader_for_microxml/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-222-yellow.svg)](https://github.com/LucianoBestia/reader_for_microxml/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-287-orange.svg)](https://github.com/LucianoBestia/reader_for_microxml/)
//!
//!
//!
//! There are many xml parsers/readers/tokenizers/lexers around, but I need something very small and simple for my simple html templates in wasm.\
//! I found the existence of a standard (or W3C proposal) for *MicroXml* - dramatically simpler then the full Xml standard. Perfect for my use-case: I have small simple html files, that are microXml compatible.  
//!
//! ## microXml
//!
//! MicroXML is a subset of XML. It is dramatically simpler.\
//! <https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/>\
//! <https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html>\
//! MicroXML is actually well-formed Xml.\
//! In the data model of MicroXml there are no CData, namespaces, declarations, processing instructions,...\
//! An example of all can be done in a well-formed microXml:  
//!
//! ```xml
//! <memo lang="en" date="2017-05-01">
//!     I <em>love</em> microXML!<br />
//!     <!-- some comment -->
//!     It's so clean &amp; simple.
//! </memo>
//! ```
//!
//! MicroXml can be only in utf-8. I am lucky, because Rust Strings are internally utf-8 and are automatically checked for correctness.\
//! MicroXml should go through normalization: CR & CRLF should be converted to LF, but I don't do that here. Also decoding xml control characters `&quot;`, `&amp;`,... or decoding unicode encodings like `&#xE343;` , `&#xE312;`,... is not inside the reader. This is left for a higher library to choose what to do with it.\
//! MicroXml can contain Comments, but they are not official microXml data. But I need them for my templating project.\
//! Whitespaces are completely preserved in Text Nodes. For me they are significant. Also newline and Tabs. This is different from full Xml whitespace processing.\
//! All other whitespaces are ignored - they are insignificant.  
//!
//! ## reader
//!
//! This ReaderForMicroXml obviously cannot read a complicated full XML.\
//! This `reader_for_microxml` is used for small html fragments.\
//! They must be well-formed microXml.\
//! This fragments are meant for a html templating for dodrio.\
//! Because of the small size of fragments, I can put all the text in memory in a string.\
//! Only basic mal-formed incorrectness produce errors. I am not trying to return errors for all the possible mal-formed incorrectness in microXml.\
//! The speed is not really important, but the size of the code is, because it will be used in WebAssembly. Every code is too big for Wasm!\
//! The crate has `#![no_std]`, #![forbid(unsafe_code)], NO dependencies, NO allocations,  
//!
//! ## iterator
//!
//! The reader is an iterator.\
//! It implements the trait of the iterator.\
//! Use this syntax to process all tokens:\
//! `for result_token in reader_iterator {`\
//! or\
//! `let x: Option<Result<Token, &str>> = reader_iterator.next();`  
//!
//! ## Tests
//!
//! Run 16 tests with:\
//! `cargo make test`
//!
//! ## Examples
//!
//! Find examples in the repository on github.\
//! Run them with:  
//! `cargo make run_rel1`\
//! `cargo make run_rel2`\
//! it is a shortcut to:\
//! `cargo run --example microxml_tree examples/t2.html`
//!
//! ```rust
//! /// read xml and write to screen
//! use reader_for_microxml::*;
//!
//! fn main(){
//!     let str_xml = r#"<html>test</html>"#;
//!     let mut reader_iterator = ReaderForMicroXml::new(str_xml);
//!     let result = read_xml_to_debug_string(&mut reader_iterator);
//!     println!("Result: {}", result)
//! }
//!
//! fn read_xml_to_debug_string(reader_iterator: &mut ReaderForMicroXml) -> String {
//!     let mut result = String::new();
//!     // reader_iterator is iterator Option<Result<Token,&str>>
//!     // the first option is used for the iterator to know where is the end
//!     // then the Result can have an Token or an Error
//!     for result_token in reader_iterator {
//!         match result_token {
//!             Ok(token) => match token {
//!                 Token::StartElement(name) => {
//!                     result.push_str(&format!("Start: \"{}\"\n", name));
//!                 }
//!                 Token::Attribute(name, value) => {
//!                     result.push_str(&format!("Attribute: \"{}\" = \"{}\"\n", name, value));
//!                 }
//!                 Token::TextNode(txt) => {
//!                     result.push_str(&format!("Text: \"{}\"\n", txt));
//!                 }
//!                 Token::Comment(txt) => {
//!                     result.push_str(&format!("Comment: \"{}\"\n", txt));
//!                 }
//!                 Token::EndElement(name) => {
//!                     result.push_str(&format!("End: \"{}\"\n", name));
//!                 }
//!             },
//!             Err(err_msg) => {
//!                 panic!(err_msg);
//!             }
//!         }
//!     }
//!     //return
//!     result
//! }
//! ```
//!
//! ## used in projects
//!
//! <https://github.com/LucianoBestia/cargo_crev_web>  
//! <https://github.com/LucianoBestia/dodrio_templating>  
//! <https://github.com/LucianoBestia/mem6_game>  
//!
//! ## cargo crev reviews and advisory
//!
//! It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)\
//! to verify the trustworthiness of each of your dependencies.\
//! Please, spread this info.\
//! On the web use this url to read crate reviews. Example:\
//! <https://web.crev.dev/rust-reviews/crate/num-traits>  
//!
//! ## Ideas for the future
//!
//! ### Speed
//!
//! The speed could probably be improved if I use Vec\<u8\> instead of CharIndices. That could work because all the xml delimiters are ASCII characters. The specifics of the UTF-8 encoding is that ASCII characters can in no way be misinterpreted inside a string. They always have the first bit set to 0.\
//! All other unicode characters are multi-byte and all this bytes MUST start with bit 1.\
//! So there is no way of having them confused.\
//! <https://betterexplained.com/articles/unicode/>\
//! <https://naveenr.net/unicode-character-set-and-utf-8-utf-16-utf-32-encoding/>  
//!
//! ## References
//!
//! <https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html>\
//! <https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/>\
//! <https://github.com/tafia/quick-xml>\
//! <https://github.com/RazrFalcon/roxmltree>  
//!
// endregion: lmake_md_to_doc_comments include README.md A //!

#![no_std]
#![forbid(unsafe_code)]

pub struct PosChar {
    pub pos: usize,
    pub ch: char,
}

/// struct Reader for MicroXml - the Class  
/// Rust has Structs + Traits, but for me it is just like Class/Object.  
/// Just without inheritance.  
/// All the fields are internal and not public.  
/// The only way to interact is through methods.  
pub struct ReaderForMicroXml<'a> {
    /// reference to the xml string (no allocation)
    input: &'a str,
    /// Iterator CharIndices over the input string
    indices: core::str::CharIndices<'a>,
    /// I need to know the TagState for programming as a state machine
    tag_state: TagState,
    /// the last read character from the indices iterator
    last_char: PosChar,
    /// for significant whitespace (in TextNode beginning)
    start_of_text_node_before_whitespace: usize,
}

/// The reader_for_microxml returns tokens.  
/// The caller will manage this tokens. So they must be public.  
/// The string slices are reference to the original string with microXml text  
#[derive(Clone, Debug)]
pub enum Token<'a> {
    /// Start of xml element  
    StartElement(&'a str),
    /// End of xml element  
    EndElement(&'a str),
    /// Attribute  
    Attribute(&'a str, &'a str),
    /// Text node between `StartElement` and `EndElement`.  
    TextNode(&'a str),
    /// comment node
    Comment(&'a str),
}

/// internal enum: Tags are strings inside delimiters `< and >  like <div> or </div>`  
enum TagState {
    /// outside of tag  
    OutsideOfTag,
    /// inside of tag  
    InsideOfTag,
    /// reached normal end of file
    EndOfFile,
}

impl PosChar {
    pub fn set(&mut self, tup: (usize, char)) {
        self.pos = tup.0;
        self.ch = tup.1;
    }
}

impl<'a> ReaderForMicroXml<'a> {
    /// Constructor. String is immutably borrowed here. No allocation.  
    pub fn new(input: &str) -> ReaderForMicroXml {
        // CharIndices is an iterator that returns a tuple: (pos, ch).
        // I convert this into PosChar{pos, ch} for easier coding.
        // The "byte" position for using the string slice and the character.
        // This is a complication because one utf-8 character can have more bytes.
        // And the slices are defined by "bytes position", not by "character position".
        // Very important distinction!

        let mut indices = input.char_indices();
        let mut last_char = PosChar { pos: 0, ch: ' ' };
        if input.is_empty() {
            // unwrap because it cannot error if the string is not empty
            last_char.set(indices.next().unwrap());
        }
        ReaderForMicroXml {
            input,
            indices,
            tag_state: TagState::OutsideOfTag,
            last_char,
            start_of_text_node_before_whitespace: 0,
        }
    }

    /// Reads the next token (internal).  
    /// The internal function can understand when the Eof is in a correct position  
    /// and stops the propagation of Option None.  
    #[allow(clippy::integer_arithmetic, clippy::nonminimal_bool)]
    fn read_token_internal(&mut self) -> Option<Result<Token<'a>, &'static str>> {
        match &self.tag_state {
            TagState::OutsideOfTag => {
                if self.start_of_text_node_before_whitespace == 0 {
                    self.start_of_text_node_before_whitespace = self.last_char.pos;
                }
                self.move_over_whitespaces()?;
                // Tags can look like this:
                // Start Tags: < xxx >,  < xxx attr="val" >,  < xxx />
                // End Tags: </xxx>
                // Comments: <!-- xxx -->
                // start delimiter is <
                if self.last_char.ch == '<' {
                    self.tag_state = TagState::InsideOfTag;
                    self.move_next_char()?;
                    self.move_over_whitespaces()?;
                    // if it is not comment or end tag, must be the element name
                    if !(self.last_char.ch == '!' || self.last_char.ch == '/') {
                        self.read_element_name()
                    } else if self.last_char.ch == '!' {
                        // this is a comment <!-- xxx -->
                        // comment are not data in MicroXml standard
                        // but I need them for my templating project
                        self.read_comment()
                    } else {
                        // the end element look like this </xxx>
                        self.read_end_element()
                    }
                } else {
                    // the text node is between element so looks like this
                    // > text <
                    self.read_text_node()
                }
            }
            TagState::InsideOfTag => {
                self.move_over_whitespaces()?;
                // InsideOfTag (after name) can be > or attributes or self_closing
                // < xxx >,  < xxx attr="val" >,  < xxx />
                // if it is not self-closing or > then must be an attribute
                if self.last_char.ch == '>' {
                    // here must be the end of start tag >
                    self.move_next_char()?;
                    self.tag_state = TagState::OutsideOfTag;
                    self.start_of_text_node_before_whitespace = 0;
                    // recursive calling
                    return self.read_token_internal();
                } else if self.last_char.ch == '/' {
                    // self-closing element
                    self.move_next_char()?; // to >
                    self.move_over_whitespaces()?;
                    if self.last_char.ch != '>' {
                        return Some(Err("Error: Tag has / but not />"));
                    } else {
                        self.move_next_char()?;
                        self.tag_state = TagState::OutsideOfTag;
                        self.start_of_text_node_before_whitespace = 0;
                        return Some(Ok(Token::EndElement("")));
                    }
                } else {
                    // attribute
                    self.read_attribute()
                }
            }
            TagState::EndOfFile => {
                //return None to stop the iterator
                None
            }
        }
    }

    /// Reads the element name  
    /// Propagation of Option None if is Eof  
    fn read_element_name(&mut self) -> Option<Result<Token<'a>, &'static str>> {
        // start of tag name < xxx >
        self.move_over_whitespaces()?;
        let start_pos = self.last_char.pos;
        let end_pos;
        loop {
            // read until delimiter space, / or >
            if self.last_char.ch.is_whitespace()
                || self.last_char.ch == '/'
                || self.last_char.ch == '>'
            {
                end_pos = self.last_char.pos;
                break;
            } else {
                self.move_next_char()?;
            }
        }
        self.move_over_whitespaces()?;
        self.tag_state = TagState::InsideOfTag;

        // unwrap because I am confident that start_pos or end_pos are correct
        return Some(Ok(Token::StartElement(
            self.input.get(start_pos..end_pos).unwrap(),
        )));
    }

    /// Reads the attribute name and value.  
    /// Return Option None if Eof.  
    fn read_attribute(&mut self) -> Option<Result<Token<'a>, &'static str>> {
        self.move_over_whitespaces()?;
        let start_pos = self.last_char.pos;
        let end_pos;
        loop {
            // delimiters are whitespace or =
            if self.last_char.ch.is_whitespace() || self.last_char.ch == '=' {
                end_pos = self.last_char.pos;
                break;
            } else {
                self.move_next_char()?;
            }
        }
        // unwrap because I am confident that start_pos or end_pos are correct
        let attr_name = self.input.get(start_pos..end_pos).unwrap();

        // region: skip delimiters: whitespace, =, "
        self.move_over_whitespaces()?;
        if self.last_char.ch == '=' {
            self.move_next_char()?;
        }
        self.move_over_whitespaces()?;
        if self.last_char.ch == '"' {
            self.move_next_char()?;
        } else {
            return Some(Err("Error: Attribute does not have the char = ."));
        }
        // endregion

        let start_pos = self.last_char.pos;
        let end_pos;
        loop {
            // end delimiter is "
            if self.last_char.ch == '"' {
                end_pos = self.last_char.pos;
                self.move_next_char()?;
                break;
            } else {
                self.move_next_char()?;
            }
        }
        self.move_over_whitespaces()?;
        // unwrap because I am confident that start_pos or end_pos are correct
        let attr_value = self.input.get(start_pos..end_pos).unwrap();
        // return
        Some(Ok(Token::Attribute(attr_name, attr_value)))
    }

    /// reads end element  
    fn read_end_element(&mut self) -> Option<Result<Token<'a>, &'static str>> {
        // end tag for element  </ xxx >
        // we are already at the / char
        self.move_next_char()?;
        self.move_over_whitespaces()?;
        let start_pos = self.last_char.pos;
        let end_pos;
        loop {
            // read until space or >
            if self.last_char.ch.is_whitespace() || self.last_char.ch == '>' {
                end_pos = self.last_char.pos;
                break;
            } else {
                self.move_next_char()?;
            }
        }
        self.move_over_whitespaces()?;
        if self.last_char.ch == '>' {
            // after the End element is possible to have a correct Eof
            if let Some(()) = self.move_next_char() {
                //dbg!(self.last_char.pos);
                self.start_of_text_node_before_whitespace = self.last_char.pos;
                if let Some(()) = self.move_over_whitespaces() {
                    self.tag_state = TagState::OutsideOfTag;
                } else {
                    self.tag_state = TagState::EndOfFile;
                }
            } else {
                self.tag_state = TagState::EndOfFile;
            }
            return Some(Ok(Token::EndElement(
                // unwrap because I am confident that start_pos or end_pos are correct
                self.input.get(start_pos..end_pos).unwrap(),
            )));
        } else {
            return Some(Err("End Element does not have > ."));
        }
    }

    /// Reads text node  
    /// I don't do any encoding/decoding here, because I need it "as is" for html templating.  
    /// I preserve all the "significant" whitespaces because I will use this for templating.  
    /// And because there is no hard standard for trailing spaces in xml text node.  
    /// If reached Eof propagates Option None.  
    fn read_text_node(&mut self) -> Option<Result<Token<'a>, &'static str>> {
        // text element look like this > some text <
        // it has significant whitespace start
        let start_pos = self.start_of_text_node_before_whitespace;
        // reset it to 0, because I don't need it more here
        // and this is the signal to store a new one.
        self.start_of_text_node_before_whitespace = 0;

        let mut end_pos;
        loop {
            //dbg!(self.last_char.ch);
            end_pos = self.last_char.pos;
            // end delimiter in < or end of file
            if self.last_char.ch == '<' {
                self.tag_state = TagState::OutsideOfTag;
                break;
            } else {
                if self.move_next_char().is_none() {
                    end_pos += 1;
                    self.tag_state = TagState::EndOfFile;
                    break;
                }
            }
        }
        // unwrap because I am confident that start_pos or end_pos are correct

        //dbg!(end_pos);
        return Some(Ok(Token::TextNode(
            self.input.get(start_pos..end_pos).unwrap(),
        )));
    }

    /// Comments are not data for MicroXml standard,  
    /// But I need them as data for my templating project.  
    /// The Option is returned only because of Option None propagation because of Eof.  
    fn read_comment(&mut self) -> Option<Result<Token<'a>, &'static str>> {
        // comments looks like this <!-- xxx -->
        // we should be now at the second character  <!
        self.move_next_char()?; // skip char !
        self.move_next_char()?; // skip char -
        self.move_next_char()?; // skip char -
        let start_pos = self.last_char.pos;
        let end_pos;
        // read until end of comment -->
        let mut ch1 = ' ';
        let mut ch2 = ' ';
        loop {
            let ch3 = self.last_char.ch;
            // end delimiter -->
            if ch1 == '-' && ch2 == '-' && ch3 == '>' {
                end_pos = self.last_char.pos - 2;
                self.move_next_char()?;
                break;
            } else {
                ch1 = ch2;
                ch2 = ch3;
                self.move_next_char()?;
            }
        }
        // it is possible to have a comment in between 2 text nodes
        self.start_of_text_node_before_whitespace = 0;
        self.tag_state = TagState::OutsideOfTag;
        // unwrap because I am confident that start_pos or end_pos are correct
        return Some(Ok(Token::Comment(
            self.input.get(start_pos..end_pos).unwrap(),
        )));
    }

    // region: methods for iterator

    /// Moves the iterator and stores the last_char.  
    /// Iterator next() of CharIndices is consuming the char.  
    /// There is no way back to the same char.  
    /// But often I need to get again the same character of the last operation.  
    /// I tried with peekable.peek(), but it gives a reference and this was a problem.  
    /// So now I have 2 separate methods: move_next_char() and get_last_char().  
    /// I store the last_char for repeated use.  
    /// Anytime it can reach the End of File (Eof),  
    /// then it propagates the Option None to the caller with the ? syntax.  
    /// Only the caller knows if the Eof here is ok or it is an unexpected error.  
    /// The usize inside the Option is only a dummy,  
    /// only because I need to propagate the Option None because of Eof  
    fn move_next_char(&mut self) -> Option<()> {
        // Eof can be reached anytime. I will propagate None to the caller with ?
        self.last_char.set(self.indices.next()?);
        // returns a dummy only because of Option None propagation with ?
        Some(())
    }

    /// Skips all whitespaces if there is any  
    /// and returns the last_char when it is not whitespace.  
    /// saves the whitespace beginning position, because
    /// the caller must know if the whitespaces are insignificant. For example TextNode.  
    /// If found Eof, propagates Option None.  
    fn move_over_whitespaces(&mut self) -> Option<()> {
        loop {
            if !self.last_char.ch.is_whitespace() {
                return Some(());
            } else {
                self.move_next_char()?;
            }
        }
    }
    // endregion
}

impl<'a> Iterator for ReaderForMicroXml<'a> {
    type Item = Result<Token<'a>, &'static str>;
    /// Reads the next token: StartElement, Attribute, Text, EndElement  
    fn next(&mut self) -> Option<Result<Token<'a>, &'static str>> {
        // return
        self.read_token_internal()
    }
}
