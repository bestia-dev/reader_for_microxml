//region: lmake_readme insert "readme.md"
//! # reader for microXml
//! 
//! *Things are changing fast. 2020-01-13 LucianoBestia ver.1.0.0.*  
//! 
//! There are many xml parsers/readers/tokenizers/lexers around, but I need something very small and simple for my simple html templates.  
//! I found the existence of a standard (or W3C proposal) for *MicroXml* - dramatically simpler then the full Xml standard. Perfect for my use-case: I have small simple html files, that are microXml compatible.  
//! 
//! ## microXml
//! 
//! MicroXML is a subset of XML. It is dramatically simpler.  
//! <https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/>  
//! <https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html>  
//! MicroXML is actually well-formed Xml.  
//! In the data model of MicroXml there are no comments, CData, namespaces, declarations, processing instructions,...  
//! An example of all can be done in a well-formed microXml:  
//! 
//! ```xml
//! <memo lang="en" date="2017-05-01">
//!     I <em>love</em> &#xB5;
//!     <!-- MICRO SIGN -->XML!<br />
//!     It's so clean &amp; simple.</memo>
//! ```
//! 
//! MicroXml can be only in utf-8. I am lucky, because Rust Strings are internally utf-8 and are automatically checked for correctness.  
//! MicroXml should go through normalization: CR & CRLF should be converted to LF, but I don't do that here.  
//! MicroXml can contain Comments, but they are not microXml data, so I just skip them.  
//! Whitespaces are completely preserved in Text Nodes. For me they are significant. Also newline and Tabs. This is different from full Xml whitespace processing.  
//! All other whitespaces are ignored - they are insignificant.  
//! 
//! ## reader
//! 
//! This ReaderForMicroXml obviously cannot read a complicated full XML.  
//! This reader_for_microxml is used for small html fragments.  
//! They must be well-formed microXml.  
//! This fragments are meant for a html templating for dodrio.  
//! Because of the small size of fragments, I can put all the text in memory in a string.  
//! Only basic mal-formed incorrectness produce errors. I am not trying to return errors for all the possible mal-formed incorrectness in microXml.  
//! The speed is not really important, but the size of the code is, because it will be used in WebAssembly. Every code is too big for Wasm!  
//! The crate has NO dependencies, NO allocations.  
//! Probably it could be also `#![no_std]`, but I don't need that.  
//! 
//! ## References
//! 
//! <https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html>  
//! <https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/>  
//! <https://github.com/tafia/quick-xml>  
//! <https://github.com/RazrFalcon/roxmltree>  
//! 


//endregion: lmake_readme insert "readme.md"

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
    /// the last read character from the iterator
    last_char: (usize, char),
}

/// The reader_for_microxml returns events.  
/// The caller will manage this events. So they must be public.  
/// The string slices are reference to the original string with microXml text  
#[derive(Clone, Debug)]
pub enum Event<'a> {
    /// Start of xml element  
    StartElement(&'a str),
    /// End of xml element  
    EndElement(&'a str),
    /// Attribute  
    Attribute(&'a str, &'a str),
    /// Child Text between `StartElement` and `EndElement`.  
    TextNode(&'a str),
    /// Error when reading  
    Error(&'static str),
    /// End of microXml document  
    Eof,
}

/// internal enum: Tags are strings inside delimiters `< and >  like <div> or </div>`  
enum TagState {
    /// outside of tag  
    OutsideOfTag,
    /// inside of tag  
    InsideOfTag,
    /// end of file  
    Eof,
}

impl<'a> ReaderForMicroXml<'_> {
    /// Constructor. String is immutably borrowed here. No allocation.  
    pub fn new(input: &str) -> ReaderForMicroXml {
        // CharIndices is an iterator that returns a tuple: (pos, ch).
        // The "byte" position for using the string slice and the character.
        // This is a complication because one utf-8 character can have more bytes.
        // And the slices are defined by "bytes position", not by "character position".
        // Very important distinction!

        let mut indices = input.char_indices();
        let mut last_char = (0, ' ');
        if input.is_empty() {
            // unwrap because it cannot error if the string is not empty
            last_char = indices.next().unwrap();
        }
        ReaderForMicroXml {
            input,
            indices,
            tag_state: TagState::OutsideOfTag,
            last_char,
        }
    }

    /// Reads the next event: StartElement, Attribute, Text, EndElement  
    /// If the Option None (Eof) has propagated till here, this is an Error.  
    pub fn read_event(&mut self) -> Event {
        match self.read_event_internal() {
            Some(x) => x,
            None => Event::Error("Eof on incorrect position."),
        }
    }

    /// Reads the next event (internal).  
    /// The internal function can understand when the Eof is in a correct position  
    /// and stops the propagation of Option None.  
    #[allow(clippy::integer_arithmetic, clippy::nonminimal_bool)]
    fn read_event_internal(&mut self) -> Option<Event> {
        match &self.tag_state {
            TagState::OutsideOfTag => {
                // I have to remember the start_pos if it is a text node
                let (pos, _ch) = self.get_last_char();
                let start_pos = pos;
                let (_pos, ch) = self.skip_whitespace_and_get_last_char()?;
                // Tags can look like this:
                // Start Tags: < xxx >,  < xxx attr="val" >,  < xxx />
                // End Tags: </xxx>
                // Comments: <!-- xxx -->
                // start delimiter is <
                if ch == '<' {
                    self.tag_state = TagState::InsideOfTag;
                    self.move_next_char()?;
                    let (_pos, ch) = self.skip_whitespace_and_get_last_char()?;
                    // if it is not comment or end tag, must be the element name
                    if !(ch == '!' || ch == '/') {
                        self.read_element_name()
                    } else if ch == '!' {
                        // this is a comment <!-- xxx -->
                        // skip the comment because it is no data in MicroXml standard
                        self.skip_comment()?;
                        // recursive calling
                        return Some(self.read_event());
                    } else {
                        // the end element look like this </xxx>
                        self.read_end_element()
                    }
                } else {
                    // the text node is between element so looks like this
                    // > text <
                    self.read_text_node(start_pos)
                }
            }
            TagState::InsideOfTag => {
                let (_pos, ch) = self.skip_whitespace_and_get_last_char()?;
                // InsideOfTag (after name) can be > or attributes or self_closing
                // < xxx >,  < xxx attr="val" >,  < xxx />
                // if it is not self-closing or > then must be an attribute
                if !(ch == '/' || ch == '>') {
                    // attribute
                    self.read_attribute()
                } else if ch == '/' {
                    // self-closing element
                    self.move_next_char()?; // to >
                    let (_pos, ch) = self.skip_whitespace_and_get_last_char()?;
                    if ch != '>' {
                        Some(Event::Error("Tag has / but not />"))
                    } else {
                        self.tag_state = TagState::OutsideOfTag;
                        self.move_next_char()?;
                        return Some(Event::EndElement(""));
                    }
                } else {
                    // here must be the end of the tag >
                    self.move_next_char()?;
                    self.tag_state = TagState::OutsideOfTag;
                    // recursive calling
                    return Some(self.read_event());
                }
            }
            TagState::Eof => {
                // End of file
                return Some(Event::Eof);
            }
        }
    }

    /// Reads the element name  
    /// Propagation of Option None if is Eof  
    fn read_element_name(&mut self) -> Option<Event> {
        // start of tag name < xxx >
        let (pos, _ch) = self.skip_whitespace_and_get_last_char()?;
        let start_pos = pos;
        let end_pos;
        loop {
            let (pos, ch) = self.get_last_char();
            // read until delimiter space, / or >
            if ch.is_whitespace() || ch == '/' || ch == '>' {
                end_pos = pos;
                break;
            } else {
                self.move_next_char()?;
            }
        }
        self.skip_whitespace_and_get_last_char()?;
        self.tag_state = TagState::InsideOfTag;

        // unwrap because I am confident that start_pos or end_pos are correct
        return Some(Event::StartElement(
            self.input.get(start_pos..end_pos).unwrap(),
        ));
    }

    /// Reads the attribute name and value.  
    /// Return Option None if Eof.  
    fn read_attribute(&mut self) -> Option<Event> {
        let (pos, _ch) = self.skip_whitespace_and_get_last_char()?;
        let start_pos = pos;
        let end_pos;
        loop {
            let (pos, ch) = self.get_last_char();
            // delimiters are whitespace or =
            if ch.is_whitespace() || ch == '=' {
                end_pos = pos;
                break;
            } else {
                self.move_next_char()?;
            }
        }
        // unwrap because I am confident that start_pos or end_pos are correct
        let attr_name = self.input.get(start_pos..end_pos).unwrap();

        // region: skip delimiters: whitespace, =, "
        let (_pos, ch) = self.skip_whitespace_and_get_last_char()?;
        if ch == '=' {
            self.move_next_char()?;
        }
        let (_pos, ch) = self.skip_whitespace_and_get_last_char()?;
        if ch == '"' {
            self.move_next_char()?;
        } else {
            return Some(Event::Error("Attribute does not have the char = ."));
        }
        // endregion

        let (pos, _ch) = self.get_last_char();
        let start_pos = pos;
        let end_pos;
        loop {
            let (pos, ch) = self.get_last_char();
            // end delimiter is "
            if ch == '"' {
                end_pos = pos;
                self.move_next_char()?;
                break;
            } else {
                self.move_next_char()?;
            }
        }
        self.skip_whitespace_and_get_last_char()?;
        // unwrap because I am confident that start_pos or end_pos are correct
        let attr_value = self.input.get(start_pos..end_pos).unwrap();
        // return
        Some(Event::Attribute(attr_name, attr_value))
    }

    /// reads end element  
    fn read_end_element(&mut self) -> Option<Event> {
        // end tag for element  </ xxx >
        // we are already at the / char
        self.move_next_char()?;
        let (pos, _ch) = self.skip_whitespace_and_get_last_char()?;
        let start_pos = pos;
        let end_pos;
        loop {
            let (pos, ch) = self.get_last_char();
            // read until space or >
            if ch.is_whitespace() || ch == '>' {
                end_pos = pos;
                break;
            } else {
                self.move_next_char()?;
            }
        }
        let (_pos, ch) = self.skip_whitespace_and_get_last_char()?;
        if ch == '>' {
            // after the End element is possible to have a correct Eof
            // everywhere else it is an error
            match self.move_next_char() {
                Some(_x) => match self.skip_whitespace_and_get_last_char() {
                    Some(_x) => {
                        self.tag_state = TagState::OutsideOfTag;
                    }
                    None => {
                        self.tag_state = TagState::Eof;
                    }
                },
                None => {
                    self.tag_state = TagState::Eof;
                }
            }
            return Some(Event::EndElement(
                // unwrap because I am confident that start_pos or end_pos are correct
                self.input.get(start_pos..end_pos).unwrap(),
            ));
        } else {
            return Some(Event::Error("End Element does not have > ."));
        }
    }

    /// Reads text node  
    /// I don't do any encoding/decoding here, because I need it "as is" for html templating.  
    /// I preserve all the "significant" whitespaces because I will use this for templating.  
    /// And because there is no hard standard for trailing spaces in xml text node.  
    /// If reached Eof propagates Option None.  
    fn read_text_node(&mut self, start_pos: usize) -> Option<Event> {
        // text element look like this > some text <
        let (_pos, _ch) = self.get_last_char();
        let end_pos;
        loop {
            let (pos, ch) = self.get_last_char();
            // end delimiter in <
            if ch == '<' {
                end_pos = pos;
                self.tag_state = TagState::OutsideOfTag;
                break;
            } else {
                self.move_next_char()?;
            }
        }
        // unwrap because I am confident that start_pos or end_pos are correct
        return Some(Event::TextNode(self.input.get(start_pos..end_pos).unwrap()));
    }

    /// Comments are not data for MicroXml standard,  
    /// They are ignored, I don't return them.  
    /// The Option is returned only because of Option None propagation because of Eof.  
    fn skip_comment(&mut self) -> Option<usize> {
        // comments looks like this <!-- xxx -->
        // we should be now at the second character  <!
        self.move_next_char()?; // skip char -
        self.move_next_char()?; // skip char -

        // read until end of comment -->
        let mut ch1 = ' ';
        let mut ch2 = ' ';
        loop {
            let (_pos, ch3) = self.get_last_char();
            // end delimiter -->
            if ch1 == '-' && ch2 == '-' && ch3 == '>' {
                self.move_next_char()?;
                break;
            } else {
                ch1 = ch2;
                ch2 = ch3;
                self.move_next_char()?;
            }
        }
        self.skip_whitespace_and_get_last_char()?;
        self.tag_state = TagState::OutsideOfTag;
        // returns a dummy only because of Option None propagation because of Eof
        Some(0)
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
    fn move_next_char(&mut self) -> Option<usize> {
        // Eof can be reached anytime. I will propagate None to the caller with ?
        self.last_char = self.indices.next()?;
        // returns a dummy only because of Option None propagation with ?
        Some(0)
    }

    /// Returns the last_char, but doesn't move the iterator.  
    /// Cannot error  
    fn get_last_char(&self) -> (usize, char) {
        self.last_char
    }

    /// Skips all whitespaces if there is any  
    /// and returns the last_char when it is not whitespace.  
    /// The caller must know prior to call that the whitespaces are insignificant.  
    /// If found Eof, propagates Option None.  
    fn skip_whitespace_and_get_last_char(&mut self) -> Option<(usize, char)> {
        loop {
            let (pos, ch) = self.get_last_char();
            if !ch.is_whitespace() {
                return Some((pos, ch));
            } else {
                self.move_next_char()?;
            }
        }
    }
    // endregion
}
