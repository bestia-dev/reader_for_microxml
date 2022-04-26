[comment]: # (lmake_md_to_doc_comments segment start A)

# reader for microXml

[comment]: # (lmake_cargo_toml_to_md start)

**reader for microXml - the simplified subset of xml**  
***version: 2.0.2  date: 2021-01-13 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/reader_for_microxml)***  

[comment]: # (lmake_cargo_toml_to_md end)

 [![crates.io](https://img.shields.io/crates/v/reader_for_microxml.svg)](https://crates.io/crates/reader_for_microxml)
 [![Documentation](https://docs.rs/reader_for_microxml/badge.svg)](https://docs.rs/reader_for_microxml/)
 [![crev reviews](https://web.crev.dev/rust-reviews/badge/crev_count/reader_for_microxml.svg)](https://web.crev.dev/rust-reviews/crate/reader_for_microxml/)
 [![RustActions](https://github.com/bestia-dev/reader_for_microxml/workflows/rust/badge.svg)](https://github.com/bestia-dev/reader_for_microxml/)
 [![latest doc](https://img.shields.io/badge/latest_docs-GitHub-orange.svg)](https://bestia-dev.github.io/reader_for_microxml/reader_for_microxml/index.html)
 [![Licence](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/bestia-dev/reader_for_microxml/blob/main/LICENSE)
[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Freader_for_microxml&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)

[comment]: # (lmake_lines_of_code start)
[![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-278-green.svg)](https://github.com/bestia-dev/reader_for_microxml/)
[![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-208-blue.svg)](https://github.com/bestia-dev/reader_for_microxml/)
[![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-64-purple.svg)](https://github.com/bestia-dev/reader_for_microxml/)
[![Lines in examples](https://img.shields.io/badge/Lines_in_examples-222-yellow.svg)](https://github.com/bestia-dev/reader_for_microxml/)
[![Lines in tests](https://img.shields.io/badge/Lines_in_tests-287-orange.svg)](https://github.com/bestia-dev/reader_for_microxml/)

[comment]: # (lmake_lines_of_code end)

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
MicroXml should go through normalization: CR & CRLF should be converted to LF, but I don't do that here. Also decoding xml control characters `&quot;`, `&amp;`,... or decoding unicode encodings like `&#xE343;` , `&#xE312;`,... is not inside the reader. This is left for a higher library to choose what to do with it.\
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
The crate has `#![no_std]`, #![forbid(unsafe_code)], NO dependencies, NO allocations,  

## iterator

The reader is an iterator.\
It implements the trait of the iterator.\
Use this syntax to process all tokens:\
`for result_token in reader_iterator {`\
or\
`let x: Option<Result<Token, &str>> = reader_iterator.next();`  

## Tests

Run 16 tests with:\
`cargo make test`

## Examples

Find examples in the repository on github.\
Run them with:  
`cargo make run_rel1`\
`cargo make run_rel2`\
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

## used in projects

<https://github.com/bestia-dev/cargo_crev_web>  
<https://github.com/bestia-dev/dodrio_templating>  
<https://github.com/bestia-dev/mem6_game>  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)\
to verify the trustworthiness of each of your dependencies.\
Please, spread this info.\
On the web use this url to read crate reviews. Example:\
<https://web.crev.dev/rust-reviews/crate/num-traits>  

## Ideas for the future

### Speed

The speed could probably be improved if I use Vec\<u8\> instead of CharIndices. That could work because all the xml delimiters are ASCII characters. The specifics of the UTF-8 encoding is that ASCII characters can in no way be misinterpreted inside a string. They always have the first bit set to 0.\
All other unicode characters are multi-byte and all this bytes MUST start with bit 1.\
So there is no way of having them confused.\
<https://betterexplained.com/articles/unicode/>\
<https://naveenr.net/unicode-character-set-and-utf-8-utf-16-utf-32-encoding/>  

## References

<https://dvcs.w3.org/hg/microxml/raw-file/tip/spec/microxml.html>\
<https://www.xml.com/articles/2017/06/03/simplifying-xml-microxml/>\
<https://github.com/tafia/quick-xml>\
<https://github.com/RazrFalcon/roxmltree>  

[comment]: # (lmake_md_to_doc_comments segment end A)
