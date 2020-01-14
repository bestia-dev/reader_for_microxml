//! reader_for_microxml.rs
//! 2020-01-12  Luciano Bestia

//Description
//MicroXML is a subset of XML. It is dramatically simpler.
//https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/
//https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html
//MicroXML is actualy well-formed Xml. But Xml is not always well-formed MicroXML.
//This ReaderForMicroXml obviously cannot read a complicated full XML.
//Limitations: only utf-8 (rust Strings are utf-8 internally and are
//automatically checked for correct utf-8),
//normalization: CRLF shoul be converted to LF, but I don't do that here.
//comments are not microxml data, so I skip them
//special difference from Xml: LF inside a Text remains (in xml is replaced with a space)

//This reader_for_microxml is used for small html fragments.
//They must be well-formed MicroXml.
//This fragments are meant for a html templating for dodrio.
//Because of the small size of fragments, I can put all the text in memory in a string.

//TODO: this library should not panic, but return error to the caller
//avoid dependencies as much as possible

//public struct (behave like an object)
pub struct ReaderForMicroXml<'a> {
    //all the fields are internal and not public
    input: &'a str,
    ///the input string to read, but as iterator CharIndices
    indices: core::str::CharIndices<'a>,
    ///and I need to know the TagState
    tagstate: TagState,
    ///sometimes is needed the last character of previous read
    //it has the position and the character
    last_char: (usize, char),
}

/// The reader_for_microxml returns events.
/// The caller will manage this events.
/// So they must be public.
/// The string slices are reference to the original string with microxml text
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
    /// Comment is not data for MicroXml
    Comment,
    /// End of XML document.
    Eof,
}

/// internal enum: Tags are strings inside delimiters < and >  like <div> or </div>
enum TagState {
    OutsideOfTag,
    InsideOfTag,
    Eof,
}

impl<'a> ReaderForMicroXml<'_> {
    ///constructor. String is borrowed here.
    pub fn new(input: &str) -> ReaderForMicroXml {
        //CharIndices is an iterator that returns a tuple:
        //the byte position for using the string slice and the character
        //this is a complication because one utf-8 character can have more bytes
        let mut indices = input.char_indices();
        let last_char = indices.next().unwrap();
        ReaderForMicroXml {
            input,
            indices,
            tagstate: TagState::OutsideOfTag,
            last_char,
        }
    }
    pub fn read_event(&mut self) -> Event {
        match self.read_event2() {
            Some(x) => x,
            None => Event::Error("Eof on incorrect position"),
        }
    }

    ///read next event
    #[allow(clippy::integer_arithmetic, clippy::nonminimal_bool)]
    pub fn read_event2(&mut self) -> Option<Event> {
        match &self.tagstate {
            TagState::OutsideOfTag => {
                //libc_println!("OutsideOfTag {}", "");
                let (_pos, ch) = self.skip_whitespace_and_peek()?;
                //start of tag < xxx >
                if ch == '<' {
                    self.tagstate = TagState::InsideOfTag;
                    self.movenext()?;
                    let (_pos, ch) = self.skip_whitespace_and_peek()?;
                    //not comment or auto-close
                    if !(ch == '!' || ch == '/') {
                        self.parse_element_name()
                    } else if ch == '!' {
                        //this is a comment
                        self.parse_comment()
                    } else {
                        //the end element </div>
                        self.parse_end_element()
                    }
                } else {
                    self.parse_text_node()
                }
            }
            TagState::InsideOfTag => {
                //libc_println!("InsideOfTag {}", "");
                //attributes can be InsideOfTag or /> for self-closing element or > for startelement
                let (_pos, ch) = self.skip_whitespace_and_peek()?;
                //libc_println!("before attr {}", ch);
                //libc_println!("ch {}", ch);
                if !(ch == '/' || ch == '>') {
                    //attribute
                    self.parse_attribute()
                } else if ch == '/' {
                    //libc_println!("self-closing element {}", ch);
                    //self-closing element
                    self.movenext()?; // to >
                    let (_pos, ch) = self.skip_whitespace_and_peek()?;
                    //libc_println!("skip_whitespace_and_peek {}", ch);
                    if ch != '>' {
                        Some(Event::Error("Tag has / but not />"))
                    } else {
                        self.tagstate = TagState::OutsideOfTag;
                        self.movenext()?;
                        return Some(Event::EndElement(""));
                    }
                } else {
                    //the end of the tag >
                    self.movenext()?;
                    self.tagstate = TagState::OutsideOfTag;
                    //recursive calling
                    return Some(self.read_event());
                }
            }
            TagState::Eof => {
                return Some(Event::Eof);
            }
        }
    }

    ///parse end element
    fn parse_end_element(&mut self) -> Option<Event> {
        //end tag for element  </ xxx >
        //we are already at the / char
        self.movenext()?;
        //read until space, / or >
        let (pos, _ch) = self.skip_whitespace_and_peek()?;
        let start_pos = pos;
        let end_pos;
        //libc_println!("start_pos {}", &start_pos);
        loop {
            let (pos, ch) = self.peek();
            if ch.is_whitespace() || ch == '>' {
                end_pos = pos;
                //libc_println!("end_pos {}", &end_pos);
                break;
            } else {
                self.movenext()?;
            }
        }
        let (_pos, ch) = self.skip_whitespace_and_peek()?;
        if ch == '>' {
            //after the End element is possible to have a correct Eof
            //everywhere else it is an error
            match self.movenext() {
                Some(_x) => match self.skip_whitespace_and_peek() {
                    Some(_x) => {
                        self.tagstate = TagState::OutsideOfTag;
                    }
                    None => {
                        self.tagstate = TagState::Eof;
                    }
                },
                None => {
                    self.tagstate = TagState::Eof;
                }
            }
            return Some(Event::EndElement(
                self.input.get(start_pos..end_pos).unwrap(),
            ));
        } else {
            return Some(Event::Error("End Element is does not have > ."));
        }
    }
    //parse text, trim before and after
    //I don't do any encoding/decoding here
    fn parse_text_node(&mut self) -> Option<Event> {
        //text element
        let (pos, _ch) = self.peek();
        let start_pos = pos;
        let mut end_pos;
        //libc_println!("text start_pos {}", &start_pos);
        loop {
            let (pos, ch) = self.peek();
            if ch == '<' {
                end_pos = pos;
                self.tagstate = TagState::OutsideOfTag;
                break;
            } else {
                self.movenext()?;
            }
        }
        //trim trailing whitespaces
        let before_trim = self.input.get(start_pos..end_pos).unwrap();
        let mut indic1 = before_trim.char_indices();
        //libc_println!("end_pos {}", &end_pos);
        //libc_println!("before_trim.len() {}", before_trim.len());
        loop {
            let (pos, ch) = indic1.next_back().unwrap();
            //libc_println!("pos ch {} {}", &pos, &ch);
            if !ch.is_whitespace() {
                end_pos = end_pos - (before_trim.len() - 1 - pos);
                //libc_println!("end_pos {}", &end_pos);
                break;
            }
        }
        return Some(Event::TextNode(self.input.get(start_pos..end_pos).unwrap()));
    }

    //comments are not data for MicroXml, It is ignored, I dont return this text
    fn parse_comment(&mut self) -> Option<Event> {
        //comment <!-- xxx -->
        //we should be at the second character now <!
        self.movenext()?; //skip -
        self.movenext()?; //skip -

        //read until end of comment -->
        let mut ch1 = ' ';
        let mut ch2 = ' ';
        loop {
            let (_pos, ch3) = self.peek();
            if ch1 == '-' && ch2 == '-' && ch3 == '>' {
                self.movenext()?;
                break;
            } else {
                ch1 = ch2;
                ch2 = ch3;
                self.movenext()?;
            }
        }
        self.skip_whitespace_and_peek()?;
        self.tagstate = TagState::OutsideOfTag;
        return Some(Event::Comment);
    }
    fn parse_element_name(&mut self) -> Option<Event> {
        //start of tag name < xxx >
        //read until space, / or >
        let (pos, _ch) = self.skip_whitespace_and_peek()?;
        let start_pos = pos;
        let end_pos;
        //libc_println!("start_pos {}", &start_pos);
        loop {
            let (pos, ch) = self.peek();
            if ch.is_whitespace() || ch == '/' || ch == '>' {
                end_pos = pos;
                //libc_println!("end_pos {}", &end_pos);
                break;
            } else {
                self.movenext()?;
            }
        }
        self.skip_whitespace_and_peek()?;
        self.tagstate = TagState::InsideOfTag;

        return Some(Event::StartElement(
            self.input.get(start_pos..end_pos).unwrap(),
        ));
    }
    fn parse_attribute(&mut self) -> Option<Event> {
        let (pos, _ch) = self.skip_whitespace_and_peek()?;
        let start_pos = pos;
        let end_pos;
        //libc_println!("attr name start_pos {}", &start_pos);
        loop {
            let (pos, ch) = self.peek();
            if ch.is_whitespace() || ch == '=' {
                end_pos = pos;
                //libc_println!("attr name end_pos {}", &end_pos);
                break;
            } else {
                self.movenext()?;
            }
        }
        let attr_name = self.input.get(start_pos..end_pos).unwrap();
        //region: skip delimiter
        let (_pos, ch) = self.skip_whitespace_and_peek()?;
        if ch == '=' {
            self.movenext()?;
        }
        let (_pos, ch) = self.skip_whitespace_and_peek()?;
        if ch == '"' {
            self.movenext()?;
        } else {
            return Some(Event::Error("Attribute does not have = ."));
        }
        //end region
        let (pos, _ch) = self.peek();
        let start_pos = pos;
        let end_pos;
        //libc_println!("attr value start_pos {}", &start_pos);
        loop {
            let (pos, ch) = self.peek();
            if ch == '"' {
                end_pos = pos;
                //libc_println!("attr value end_pos {}", &end_pos);
                self.movenext()?;
                break;
            } else {
                self.movenext()?;
            }
        }
        self.skip_whitespace_and_peek()?;
        let attr_value = self.input.get(start_pos..end_pos).unwrap();
        Some(Event::Attribute(attr_name, attr_value))
    }
    //if the last_char is not whitespace it just return it
    fn skip_whitespace_and_peek(&mut self) -> Option<(usize, char)> {
        //libc_println!("skip_whitespace{}","");
        loop {
            let (pos, ch) = self.peek();
            if !ch.is_whitespace() {
                //libc_println!("!ch.is_whitespace(){}","");
                return Some((pos, ch));
            } else {
                self.movenext()?;
            }
        }
    }

    //iterator next() is consuming the char. There is no way back.
    //But often I need to get again the same character of the last operation.
    //The peekable.peek() sounded good, but it gives a reference and this was a problem.
    //So now they are 2 separate methods: movenext() and peek().

    //moves the iterator ans stores the values,
    //I dont need the value here, to keep logic simple.
    fn movenext(&mut self) -> Option<usize> {
        //Eof can be anytime. I will propagate None to the caller with ?
        self.last_char = self.indices.next()?;
        //return a dummy
        Some(1)
    }

    //peek the next char, but does not move the iterator
    fn peek(&self) -> (usize, char) {
        self.last_char
    }
}
