# reader for microXml

*Things are changing fast. 2020-01-13 LucianoBestia ver.1.0.0.*  

There are many xml parsers/readers/tokenizers/lexers around, but I need something very small and simple for my simple html templates.  
I found the existence of a standard (or W3C proposal) for *MicroXml* - dramatically simpler then the full Xml standard. Perfect for my use-case: I have small simple html files, that are microXml compatible.  

## microXml

MicroXML is a subset of XML. It is dramatically simpler.  
<https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/>  
<https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html>  
MicroXML is actually well-formed Xml.  
In the data model of MicroXml there are no comments, CData, namespaces, declarations, processing instructions,...  
An example of all can be done in a well-formed microXml:  

```xml
<memo lang="en" date="2017-05-01">
    I <em>love</em> &#xB5;
    <!-- MICRO SIGN -->XML!<br />
    It's so clean &amp; simple.</memo>
```

MicroXml can be only in utf-8. I am lucky, because Rust Strings are internally utf-8 and are automatically checked for correctness.  
MicroXml should go through normalization: CR & CRLF should be converted to LF, but I don't do that here.  
MicroXml can contain Comments, but they are not microXml data, so I just skip them.  
Whitespaces are completely preserved in Text Nodes. For me they are significant. Also newline and Tabs. This is different from full Xml whitespace processing.  
All other whitespaces are ignored - they are insignificant.  

## reader

This ReaderForMicroXml obviously cannot read a complicated full XML.  
This reader_for_microxml is used for small html fragments.  
They must be well-formed microXml.  
This fragments are meant for a html templating for dodrio.  
Because of the small size of fragments, I can put all the text in memory in a string.  
Only basic mal-formed incorrectness produce errors. I am not trying to return errors for all the possible mal-formed incorrectness in microXml.  
The speed is not really important, but the size of the code is, because it will be used in WebAssembly. Every code is too big for Wasm!  
The crate has NO dependencies, NO allocations.  
Probably it could be also `#![no_std]`, but I don't need that.  

## Examples

Find examples how to use it in the repository on github.  
Go to the /example/ folder.  
<https://github.com/LucianoBestia/reader_for_microxml>  

```rust
/// read xml and write to screen
pub fn read_and_print(input: &str) -> Result<(), String> {
    let mut pp = ReaderForMicroXml::new(input);
    println!("\n{}\n\n", input);
    loop {
        match pp.read_event() {
            Event::StartElement(name) => {
                println!("Start Element name=\"{}\"", name);
            }
            Event::Attribute(name, value) => {
                println!("Attribute name=\"{}\" value=\"{}\"", name, value);
            }
            Event::TextNode(txt) => {
                println!("Text \"{}\"", txt);
            }
            Event::EndElement(name) => {
                println!("End Element name=\"{}\"", name);
            }
            Event::Error(error_msg) => {
                return Err(format!("Error: {}", error_msg));
            }
            Event::Eof => {
                println!("Eof {}", "");
                return Ok(());
            }
        }
    }
}
```

## References

<https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html>  
<https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/>  
<https://github.com/tafia/quick-xml>  
<https://github.com/RazrFalcon/roxmltree>  
