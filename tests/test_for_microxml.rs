//! test_for_microxml
use reader_for_microxml::*;

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

#[test]
/// test simple node
fn test_01() {
    let str_xml = r#"<html>test</html>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Text: "test"
End: "html"
"#
    );
}
#[test]
/// test attribute
fn test_01a() {
    let str_xml = r#"<html class="cl_name">test</html>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Attribute: "class" = "cl_name"
Text: "test"
End: "html"
"#
    );
}

#[test]
/// todo: return only TextNode
fn test_01e1() {
    let str_xml = r#"test"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Text: "test"
"#
    );
}

#[test]
/// it does not need to start with an element node
fn test_01e1a() {
    let str_xml = r#"this<html>test</html>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Text: "this"
Start: "html"
Text: "test"
End: "html"
"#
    );
}

#[test]
#[should_panic]
fn test_01e2() {
    let str_xml = r#"<html no_good >test</html>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let _result = read_xml_to_debug_string(&mut reader_iterator);
}

#[test]
/// it is allowed more than one root element - a xml fragment
fn test_01e3() {
    let str_xml = r#"<html>test</html><xml>two</xml>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Text: "test"
End: "html"
Start: "xml"
Text: "two"
End: "xml"
"#
    );
}

#[test]
/// the reader_iterator does not panic if there are missing end tags
fn test_01e4() {
    let str_xml = r#"<html><div><div>test</html>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Start: "div"
Start: "div"
Text: "test"
End: "html"
"#
    );
}

#[test]
/// reader_iterator does not check if beginning and end tag have the same name!
/// this should be done in a higher library
fn test_01e6() {
    let str_xml = r#"<html>test</xxx>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Text: "test"
End: "xxx"
"#
    );
}

#[test]
fn test_02() {
    let str_xml = r#"<html> test </html>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Text: " test "
End: "html"
"#
    );
}

#[test]
fn test_03() {
    let str_xml = r#"< html >test< / html >"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Text: "test"
End: "html"
"#
    );
}

#[test]
fn test_04() {
    let str_xml = r#"<
     html
      >test<
       / 
       html
        >"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Text: "test"
End: "html"
"#
    );
}

#[test]
fn test_05() {
    let str_xml = r#"<html>test<bold>strong</bold>normal</html>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Text: "test"
Start: "bold"
Text: "strong"
End: "bold"
Text: "normal"
End: "html"
"#
    );
}

#[test]
fn test_06() {
    let str_xml = r#"< html > test < bold > strong < / bold > normal < / html >"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Text: " test "
Start: "bold"
Text: " strong "
End: "bold"
Text: " normal "
End: "html"
"#
    );
}

#[test]
fn test_07() {
    let str_xml = r#"<html>q<div>second<div>third<div>test<bold>strong</bold>normal</div>second</div>third</div>q</html>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Text: "q"
Start: "div"
Text: "second"
Start: "div"
Text: "third"
Start: "div"
Text: "test"
Start: "bold"
Text: "strong"
End: "bold"
Text: "normal"
End: "div"
Text: "second"
End: "div"
Text: "third"
End: "div"
Text: "q"
End: "html"
"#
    );
}

#[test]
/// self-closing elements
fn test_08() {
    let str_xml = r#"<html>one<br />two<br/>three</html>"#;
    let mut reader_iterator = ReaderForMicroXml::new(str_xml);
    let result = read_xml_to_debug_string(&mut reader_iterator);
    assert_eq!(
        result,
        r#"Start: "html"
Text: "one"
Start: "br"
End: ""
Text: "two"
Start: "br"
End: ""
Text: "three"
End: "html"
"#
    );
}
