# reader for microXml

[![crev reviews](
https://bestia.dev/cargo_crev_web/badge/crev_count/reader_for_microxml.svg
)](https://bestia.dev/cargo_crev_web/crate/reader_for_microxml/) [![Lines of code](https://tokei.rs/b1/github/LucianoBestia/reader_for_microxml?category=code)](https://github.com/LucianoBestia/reader_for_microxml) [![Rust](https://github.com/LucianoBestia/reader_for_microxml/workflows/Rust/badge.svg)](https://github.com/LucianoBestia/reader_for_microxml/actions/) [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md) [![crates.io](https://meritbadge.herokuapp.com/reader_for_microxml)](https://crates.io/crates/reader_for_microxml)  

[comment]: # (lmake_readme cargo.toml data start)
version: 1.1.6  date: 2020-05-29 authors: Luciano Bestia\
**reader for microXml - the simplified subset of xml**

[comment]: # (lmake_readme cargo.toml data end)

There are many xml parsers/readers/tokenizers/lexers around, but I need something very small and simple for my simple html templates in wasm.\
I found the existence of a standard (or W3C proposal) for *MicroXml* - dramatically simpler then the full Xml standard. Perfect for my use-case: I have small simple html files, that are microXml compatible.  

## microXml

MicroXML is a subset of XML. It is dramatically simpler.\
<https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/>\
<https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html>\
MicroXML is actually well-formed Xml.\
In the data model of MicroXml there are no CData, namespaces, declarations, processing instructions,...\
An example of all can be done in a well-formed microXml:  

```xml
<memo lang="en" date="2017-05-01">
    I <em>love</em> microXML!<br />
    <!-- some comment -->
    It's so clean &amp; simple.
</memo>
```

MicroXml can be only in utf-8. I am lucky, because Rust Strings are internally utf-8 and are automatically checked for correctness.\
MicroXml should go through normalization: CR & CRLF should be converted to LF, but I don't do that here. Also decoding xml control characters &quot;, &amp;,... or decoding unicode encodings like &#xE343; , &#xE312;,... is not inside the reader. This is left for a higher library to choose what to do with it.\
MicroXml can contain Comments, but they are not official microXml data. But I need them for my templating project.\
Whitespaces are completely preserved in Text Nodes. For me they are significant. Also newline and Tabs. This is different from full Xml whitespace processing.\
All other whitespaces are ignored - they are insignificant.  

## reader

This ReaderForMicroXml obviously cannot read a complicated full XML.\
This `reader_for_microxml` is used for small html fragments.\
They must be well-formed microXml.\
This fragments are meant for a html templating for dodrio.\
Because of the small size of fragments, I can put all the text in memory in a string.\
Only basic mal-formed incorrectness produce errors. I am not trying to return errors for all the possible mal-formed incorrectness in microXml.\
The speed is not really important, but the size of the code is, because it will be used in WebAssembly. Every code is too big for Wasm!\
The crate has `#![no_std]`, NO dependencies, NO allocations, .  

## iterator

The reader is an iterator.\
It implements the trait of the iterator.\
Use this syntax to process all tokens:\
`for result_token in reader_iterator {`\
or\
`let x: Option<Result<Token, &str>> = reader_iterator.next();`  

## Possible enhancements

### Speed

The speed could probably be improved if I use Vec\<u8\> instead of CharIndices. That could work because all the xml delimiters are ASCII characters. The specifics of the UTF-8 encoding is that ASCII characters can in no way be misinterpreted inside a string. They always have the first bit set to 0.\
All other unicode characters are multi-byte and all this bytes MUST start with bit 1.\
So there is no way of having them confused.\
<https://betterexplained.com/articles/unicode/>\
<https://naveenr.net/unicode-character-set-and-utf-8-utf-16-utf-32-encoding/>  

## Tests

Run the tests with:\
`clear; cargo make test`

## Examples

Find examples in the repository on github.\
Run them with:  
`clear; cargo make run_rel1`\
`clear; cargo make run_rel2`\
it is a shortcut to:\
`cargo run --example microxml_tree examples/t2.html`

```rust
/// read xml and write to screen
use reader_for_microxml::*;

fn main(){
    let str_xml = r#"<html>test</html>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    println!("Result: {}", result)
}

fn read_xml_to_debug_string(reader_iterator: &mut ReaderForMicroXml) -> String {
    let mut result = String::new();
    // reader_iterator is iterator Option<Result<Token,&str>>
    // the first option is used for the iterator to know where is the end
    // then the Result can have an Token or an Error
    for result_token in reader_iterator {
        match result_token {
            Ok(token) => match token {
                Token::StartElement(name) => {
                    result.push_str(&format!("Start: \"{}\"\n", name));
                }
                Token::Attribute(name, value) => {
                    result.push_str(&format!("Attribute: \"{}\" = \"{}\"\n", name, value));
                }
                Token::TextNode(txt) => {
                    result.push_str(&format!("Text: \"{}\"\n", txt));
                }
                Token::Comment(txt) => {
                    result.push_str(&format!("Comment: \"{}\"\n", txt));
                }
                Token::EndElement(name) => {
                    result.push_str(&format!("End: \"{}\"\n", name));
                }
            },
            Err(err_msg) => {
                panic!(err_msg);
            }
        }
    }
    //return
    result
}
```

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)\
to verify the trustworthiness of each of your dependencies.\
Please, spread this info.\
On the web use this url to read crate reviews. Example:\
<https://bestia.dev/cargo_crev_web/query/num-traits>  

## References

<https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html>\
<https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/>\
<https://github.com/tafia/quick-xml>\
<https://github.com/RazrFalcon/roxmltree>  
